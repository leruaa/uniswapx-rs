use alloy::primitives::{Address, Bytes, B256};
use serde::Deserialize;

use super::OrderStatus;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct OrderNotification {
    pub order_hash: B256,
    pub created_at: u64,
    pub signature: Bytes,
    pub order_status: OrderStatus,
    pub encoded_order: Bytes,
    pub chain_id: u64,
    pub filler: Option<Address>,
    pub quote_id: Option<String>,
    pub offerer: Option<Address>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
}
