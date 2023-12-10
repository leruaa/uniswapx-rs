use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Future, TryFutureExt};
use reqwest::Client as HttpClient;
use thiserror::Error;
use tower::Service;

use crate::types::{OrderPayload, OrderResponse, OrderResponseError, OrdersRequest};

pub struct OrdersService {
    http_client: HttpClient,
    url: String,
}

impl OrdersService {
    pub fn new(url: String) -> Self {
        Self {
            http_client: HttpClient::new(),
            url,
        }
    }
}

impl Service<OrdersRequest> for OrdersService {
    type Response = OrderPayload;

    type Error = OrdersError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: OrdersRequest) -> Self::Future {
        let fut = self
            .http_client
            .get(self.url.clone())
            .query(&req)
            .send()
            .map_err(Into::into)
            .and_then(|response| {
                response
                    .json::<OrderResponse>()
                    .map_ok(|response| Result::from(response).map_err(Into::into))
                    .unwrap_or_else(|err| Err(OrdersError::from(err)))
            });

        Box::pin(fut)
    }
}

#[derive(Error, Debug)]
pub enum OrdersError {
    #[error(transparent)]
    Send(#[from] reqwest::Error),
    #[error(transparent)]
    UniswapX(#[from] OrderResponseError),
}
