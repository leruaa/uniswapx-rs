use std::{fmt::Display, time::Duration};

use futures::{
    future::Either,
    stream::{self, iter, once},
    Stream, StreamExt,
};

use thiserror::Error;
use tower::{Service, ServiceBuilder, ServiceExt};

use crate::{
    orders_service::OrdersService,
    types::{Order, OrderStatus, OrdersRequest},
    OrdersError,
};

pub fn orders_stream(
    url: String,
    request: OrdersRequest,
) -> impl Stream<Item = Result<Order, StreamError>> {
    let orders_service = ServiceBuilder::new()
        .rate_limit(1, Duration::from_secs(3))
        .service(OrdersService::new(url));

    stream::unfold(
        (orders_service, request),
        |(mut orders_service, mut current_request)| async move {
            match orders_service.ready().await {
                Ok(service) => match service.call(current_request.clone()).await {
                    Ok(payload) => {
                        if let Some(order_status) = &current_request.order_status {
                            if matches!(order_status, OrderStatus::Filled) {
                                let next_request_cursor = payload
                                    .cursor
                                    .or(current_request.cursor.clone())
                                    .or(current_request.cursor_from_order(payload.orders.last()));

                                current_request = current_request.with_cursor(next_request_cursor);
                            }
                        }

                        Some((Ok(payload.orders), (orders_service, current_request)))
                    }
                    Err(err) => Some((
                        Err(StreamError::new(err, current_request.clone())),
                        (orders_service, current_request),
                    )),
                },
                Err(err) => Some((
                    Err(StreamError::new(err, current_request.clone())),
                    (orders_service, current_request),
                )),
            }
        },
    )
    .map(|x| {
        x.map_or_else(
            |err| Either::Left(once(async { Err(err) })),
            |orders| Either::Right(iter(orders).map(Ok)),
        )
    })
    .flatten()
}

#[derive(Error, Debug)]
pub struct StreamError {
    source: OrdersError,
    current_request: OrdersRequest,
}

impl StreamError {
    pub fn new(source: OrdersError, current_request: OrdersRequest) -> Self {
        Self {
            source,
            current_request,
        }
    }
}

impl Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.source, self.current_request)
    }
}
