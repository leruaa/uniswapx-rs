use std::{env, sync::Arc};

use dotenv::dotenv;
use ethers::{
    providers::{Http, Provider},
    types::BlockNumber,
};
use uniswapx::ReactorClient;

#[tokio::test]
async fn test_price() {
    dotenv().ok();
    let eth_rpc = env::var("ETH_RPC").unwrap();

    let provider = Arc::new(Provider::<Http>::try_from(eth_rpc).unwrap());
    let reactor_client = ReactorClient::new(provider, 1);

    let events = reactor_client
        .get_fill_events(
            BlockNumber::Number(18270815.into()),
            Some(BlockNumber::Number(18270840.into())),
        )
        .await;

    println!("{events:#?}");
}
