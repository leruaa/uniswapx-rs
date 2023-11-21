use std::{
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Error;
use futures::{Future, TryFutureExt};
use reqwest::Client as HttpClient;
use tower::Service;

use crate::types::{OrderResponse, OrdersRequest};

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
    type Response = OrderResponse;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: OrdersRequest) -> Self::Future {
        let fut = self
            .http_client
            .get(self.url.clone())
            .query(&req)
            .send()
            .and_then(|response| response.json::<OrderResponse>())
            .map_err(Error::from);

        Box::pin(fut)
    }
}
