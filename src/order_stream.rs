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
    types::{Order, OrdersRequest},
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
        |(mut orders_service, current_request)| async move {
            match orders_service.ready().await {
                Ok(service) => match service.call(current_request.clone()).await {
                    Ok(payload) => Some((
                        Ok(payload.orders),
                        (orders_service, current_request.with_cursor(payload.cursor)),
                    )),
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
