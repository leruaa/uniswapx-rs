use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use anyhow::Result;
use futures::{FutureExt, Stream};
use pin_project::pin_project;
use tower::{limit::RateLimit, Service, ServiceBuilder};

use crate::{
    orders_service::OrdersService,
    types::{Order, OrderResponse, OrdersRequest},
};

#[pin_project]
pub struct OrderStream {
    orders_service: RateLimit<OrdersService>,
    request: OrdersRequest,
    cursor: Option<String>,
    buffer: VecDeque<Order>,
}

impl OrderStream {
    pub fn new(url: String, request: OrdersRequest) -> Self {
        let orders_service = ServiceBuilder::new()
            .rate_limit(1, Duration::from_secs(3))
            .service(OrdersService::new(url));

        Self {
            orders_service,
            request,
            cursor: None,
            buffer: VecDeque::new(),
        }
    }
}

impl Stream for OrderStream {
    type Item = Result<Order>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.orders_service.poll_ready(cx) {
            Poll::Ready(_) => match this.buffer.pop_front() {
                Some(next) => Poll::Ready(Some(Ok(next))),
                None => {
                    let request = this.request.with_cursor(this.cursor.to_owned());
                    let mut fut = this.orders_service.call(request);

                    match fut.poll_unpin(cx) {
                        Poll::Ready(response) => match response {
                            Ok(response) => match response {
                                OrderResponse::Orders { orders, cursor } => {
                                    *this.cursor = cursor;

                                    orders.into_iter().for_each(|o| this.buffer.push_back(o));

                                    Poll::Ready(this.buffer.pop_front().map(Ok))
                                }
                                OrderResponse::Error(err) => Poll::Ready(Some(Err(err.into()))),
                            },
                            Err(err) => Poll::Ready(Some(Err(err))),
                        },
                        Poll::Pending => Poll::Pending,
                    }
                }
            },
            Poll::Pending => Poll::Pending,
        }
    }
}
