use alloy_primitives::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettledAmount {
    pub token_in: Option<Address>,
    pub amount_in: Option<U256>,
    pub token_out: Address,
    pub amount_out: U256,
}
