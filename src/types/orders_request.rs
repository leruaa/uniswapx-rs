use serde::Serialize;

use super::OrderStatus;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct OrdersRequest {
    pub chain_id: u64,
    pub order_status: Option<OrderStatus>,
    pub cursor: Option<String>,
}

impl OrdersRequest {
    pub fn with_cursor(&self, cursor: Option<String>) -> Self {
        Self {
            chain_id: self.chain_id,
            order_status: self.order_status.clone(),
            cursor,
        }
    }
}
