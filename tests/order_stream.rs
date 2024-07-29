use std::pin::pin;

use alloy::primitives::hex;
use futures::StreamExt;
use uniswapx::{
    orders_stream,
    types::{OrderStatus, OrderType, OrdersRequest},
};

#[tokio::test]
async fn test_order_stream() {
    let request = OrdersRequest {
        chain_id: Some(1),
        ..Default::default()
    };
    let mut orders_stream =
        orders_stream(String::from("https://api.uniswap.org/v2/orders"), request);

    let mut orders_stream = pin!(orders_stream);

    let next = orders_stream
        .next()
        .await
        .expect("The stream should return an order")
        .unwrap();

    assert_eq!(
        next.order_hash,
        hex!("e632accbc66d256b06a1dc086674c3e1ad35389f4a5092844514297fd2696fc9"),
        "The stream should return the oldest order"
    );
}

#[tokio::test]
async fn test_open_order_stream() {
    let request = OrdersRequest {
        chain_id: Some(42161), //42161
        order_type: Some(OrderType::DutchV1V2),
        order_status: Some(OrderStatus::Open),
        ..Default::default()
    };
    let mut orders_stream =
        orders_stream(String::from("https://api.uniswap.org/v2/orders"), request);

    let mut orders_stream = pin!(orders_stream);

    while let Some(order) = orders_stream.next().await {
        println!("{order:#?}");
    }
}
