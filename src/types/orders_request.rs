use serde::Serialize;

use super::OrderStatus;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersRequest {
    pub chain_id: Option<u64>,
    pub order_status: Option<OrderStatus>,
    pub order_hash: Option<String>,
    pub cursor: Option<String>,
}

impl OrdersRequest {
    pub fn with_cursor(&self, cursor: Option<String>) -> Self {
        Self {
            chain_id: self.chain_id,
            order_status: self.order_status.clone(),
            order_hash: self.order_hash.clone(),
            cursor,
        }
    }
}
