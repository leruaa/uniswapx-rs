use ethers::types::{serde_helpers::deserialize_stringified_numeric, Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderOutput {
    pub recipient: Address,
    #[serde(deserialize_with = "deserialize_stringified_numeric")]
    pub start_amount: U256,
    #[serde(deserialize_with = "deserialize_stringified_numeric")]
    pub end_amount: U256,
    pub token: Address,
}
