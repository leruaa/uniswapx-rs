use std::{env, sync::Arc};

use alloy_providers::provider::Provider;
use alloy_transport::BoxTransport;
use alloy_transport_http::Http;
use dotenv::dotenv;
use uniswapx::ReactorClient;

#[tokio::test]
async fn test_get_fill_events() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_RPC").unwrap();

    let transport = Http::<reqwest::Client>::new(eth_rpc.parse().unwrap());
    let transport = BoxTransport::new(transport);
    let provider = Arc::new(Provider::new(transport));
    let reactor_client = ReactorClient::new(1);

    let events = reactor_client
        .get_fill_events(provider, 18270815, Some(18270840))
        .await;

    println!("{events:#?}");
}
