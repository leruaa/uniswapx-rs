use std::env;

use alloy_json_rpc::Id;
use alloy_pubsub::PubSubConnect;
use alloy_transport_ws::WsConnect;
use dotenv::dotenv;
use futures::StreamExt;
use uniswapx::ReactorClient;

#[tokio::test]
async fn test_get_fill_events_stream() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let eth_rpc = env::var("ETH_WS_RPC").unwrap();

    let connect = WsConnect {
        url: eth_rpc,
        auth: None,
    };

    let front_end = connect.into_service().await.unwrap();

    let reactor_client = ReactorClient::new(1);

    let mut stream = reactor_client
        .get_fill_events_stream(&front_end, Id::None)
        .await
        .unwrap();

    let v = stream.next().await;

    println!("{v:?}")
}
