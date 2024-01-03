use std::{env, sync::Arc};

use alloy_providers::provider::Provider;
use alloy_rpc_client::RpcClient;
use dotenv::dotenv;
use uniswapx::ReactorClient;

#[tokio::test]
async fn test_get_fill_events() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_RPC").unwrap();

    let provider = Provider::new_with_client(
        RpcClient::builder()
            .reqwest_http(eth_rpc.parse().unwrap())
            .boxed(),
    );
    let reactor_client = ReactorClient::new(1);

    let events = reactor_client
        .get_fill_events(Arc::new(provider), 18270815, Some(18270840))
        .await;

    println!("{events:#?}");
}
