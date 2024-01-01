use alloy_primitives::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderOutput {
    pub recipient: Address,
    pub start_amount: U256,
    pub end_amount: U256,
    pub token: Address,
}
