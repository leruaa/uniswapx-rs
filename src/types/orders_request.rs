use serde::Serialize;
use serde_json::to_string;

use super::{Order, OrderStatus};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersRequest {
    pub chain_id: Option<u64>,
    pub order_status: Option<OrderStatus>,
    pub order_hash: Option<String>,
    pub cursor: Option<String>,
}

impl OrdersRequest {
    pub fn with_cursor(mut self, cursor: Option<String>) -> Self {
        self.cursor = cursor;
        self
    }
    /// We need to build a cursor form the last known order. For instance:
    /// {"chainId_orderStatus":"1_filled","createdAt":1685115350,"orderHash":"0x8b984116a793011c9288f00ce0e3a5eb5bee9234e006de154551bc915d676654"}
    pub fn or_with_cursor_from_order(mut self, order: Option<&Order>) -> Self {
        if self.cursor.is_none() {
            let (filter_key, filter_value) = match (self.chain_id, &self.order_status) {
                (None, None) => panic!("At least one of chain id or order status"),
                (None, Some(order_status)) => ("orderStatus", to_string(&order_status).unwrap()),
                (Some(chain_id), None) => ("chainId", chain_id.to_string()),
                (Some(chain_id), Some(order_status)) => (
                    "chainId_orderStatus",
                    format!("{chain_id}_{}", order_status),
                ),
            };
            self.cursor = order.map(|o| {
                format!(
                    r#"{{"{filter_key}":"{filter_value}","createdAt":{},"orderHash":"{}"}}"#,
                    o.created_at, o.order_hash
                )
            })
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::hex;

    use crate::types::{Order, OrderStatus};

    use super::OrdersRequest;

    #[test]
    fn test_or_with_cursor_from_order() {
        let request = OrdersRequest {
            chain_id: Some(1),
            order_status: Some(OrderStatus::Filled),
            ..Default::default()
        };

        let order = Order {
            chain_id: 1,
            order_hash: hex!("00000000000000000000000000000000000000000000000000000000000000aa")
                .into(),
            ..Default::default()
        };

        let request = request.or_with_cursor_from_order(Some(&order));

        assert_eq!(
            request.cursor,
            Some(String::from(
                r#"{"chainId_orderStatus":"1_filled","createdAt":0,"orderHash":"0x00000000000000000000000000000000000000000000000000000000000000aa"}"#
            ))
        );
    }
}
