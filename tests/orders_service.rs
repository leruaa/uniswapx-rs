use alloy::primitives::hex;
use tower::Service;
use uniswapx::{types::OrdersRequest, OrdersService};

#[tokio::test]
async fn test_orders_service() {
    let mut orders_services = OrdersService::new(String::from("https://api.uniswap.org/v2/orders"));
    let request = OrdersRequest {
        order_hash: Some(String::from(
            "0x33e043036e9323080855ee3011f720db6a315388dc6cfe5a9597b52188845d85",
        )),
        ..Default::default()
    };

    let payload = orders_services.call(request).await.unwrap();

    assert_eq!(payload.orders.len(), 1, "There should be 1 order");
    assert_eq!(
        payload.orders.first().unwrap().order_hash,
        hex!("33e043036e9323080855ee3011f720db6a315388dc6cfe5a9597b52188845d85"),
        "The order hash should be the one requested"
    );
}
