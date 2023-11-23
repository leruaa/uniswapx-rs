use std::time::Duration;

use anyhow::Result;
use futures::{
    stream::{self, iter},
    Stream, StreamExt, TryStreamExt,
};

use tower::{Service, ServiceBuilder, ServiceExt};

use crate::{
    orders_service::OrdersService,
    types::{Order, OrderResponse, OrdersRequest},
};

pub fn orders_stream(url: String, request: OrdersRequest) -> impl Stream<Item = Result<Order>> {
    let orders_service = ServiceBuilder::new()
        .rate_limit(1, Duration::from_secs(3))
        .service(OrdersService::new(url));

    stream::try_unfold(
        (orders_service, request),
        |(mut orders_service, current_request)| async move {
            match orders_service.ready().await {
                Ok(service) => match service.call(current_request.clone()).await {
                    Ok(response) => match response {
                        OrderResponse::Orders { orders, cursor } => Ok(Some((
                            orders,
                            (orders_service, current_request.with_cursor(cursor)),
                        ))),
                        OrderResponse::Error(err) => Err(err.into()),
                    },
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            }
        },
    )
    .map_ok(|orders| iter(orders).map(Ok))
    .try_flatten()
}
