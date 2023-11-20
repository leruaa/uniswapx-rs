use ethers::{
    abi::Address,
    types::{serde_helpers::deserialize_stringified_numeric, U256},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInput {
    #[serde(deserialize_with = "deserialize_stringified_numeric")]
    pub start_amount: U256,
    #[serde(deserialize_with = "deserialize_stringified_numeric")]
    pub end_amount: U256,
    pub token: Address,
}
