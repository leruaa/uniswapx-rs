use alloy::primitives::{Bytes, B256};
use anyhow::{bail, Error};
use serde::Deserialize;

use super::{DutchOrder, OrderInput, OrderOutput, OrderStatus, OrderType, SettledAmount};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub chain_id: u64,
    pub order_hash: B256,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub order_status: OrderStatus,
    pub input: OrderInput,
    pub outputs: Vec<OrderOutput>,
    pub encoded_order: Bytes,
    pub signature: Bytes,
    pub tx_hash: Option<String>,
    pub settled_amounts: Option<Vec<SettledAmount>>,
    pub created_at: u64,
}

impl TryFrom<&Order> for DutchOrder {
    type Error = Error;

    fn try_from(order: &Order) -> Result<Self, Self::Error> {
        match &order.order_type {
            OrderType::Dutch => DutchOrder::try_from_v1(&order.encoded_order),
            OrderType::DutchV2 => DutchOrder::try_from_v2(&order.encoded_order),
            ty => bail!("Order of type '{ty:?}' can't be decoded"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Order;

    #[test]
    fn deserialize_order() {
        let data = r#"
        {
            "outputs":[
               {
                  "recipient":"0xb8bff65b2eeb60d6b37312ca0740a742d5e7f955",
                  "startAmount":"16226997558481172",
                  "endAmount":"16226997558481172",
                  "token":"0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
               }
            ],
            "encodedOrder":"0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000064b51b750000000000000000000000000000000000000000000000000000000064b51b75000000000000000000000000b507d4ef5ed7a01e37cb578f497329cdb3c273a50000000000000000000000000000000000000000000000000000000000002710000000000000000000000000111111111117dc0aa78b770fa6a738034120c3020000000000000000000000000000000000000000000000056bc75e2d631000000000000000000000000000000000000000000000000000056bc75e2d631000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000e80bf394d190851e215d5f67b67f8f5a52783f1e000000000000000000000000b8bff65b2eeb60d6b37312ca0740a742d5e7f95500000000000000000000000000000000000000000000000000000189635c5eac0000000000000000000000000000000000000000000000000000000064b51b75000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000039a65e493e91140000000000000000000000000000000000000000000000000039a65e493e9114000000000000000000000000b8bff65b2eeb60d6b37312ca0740a742d5e7f955",
            "signature":"0x3dd85ac7743719d3d5275e21062ef8e2d98acbf8b5ceb1e0436c3dc70cb16d812de7df3c9e15dc56295d563b2c36798155dae63abb074df6aa3d6e1cdc6257c91b",
            "input":{
               "endAmount":"100000000000000000000",
               "token":"0x111111111117dC0aa78b770fA6A738034120C302",
               "startAmount":"100000000000000000000"
            },
            "orderStatus":"open",
            "createdAt":1689589146,
            "chainId":1,
            "orderHash":"0xb057f8a9f0edcd0bd7156015232785cdc2c4d8a1e84be06169f4681d483b6709",
            "type":"Dutch"
         }"#;

        // Parse the string of data into serde_json::Value.
        let order = serde_json::from_str::<Order>(data).unwrap();

        assert_eq!(
            order.input.start_amount.to_string(),
            "100000000000000000000"
        );
    }
}
