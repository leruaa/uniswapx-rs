use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

use crate::types::order::NonlinearDutchDecay;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderOutput {
    DutchV2(DutchOutput),
    DutchV3(DutchOutputV3),
}

impl OrderOutput {
    pub fn start_amount(&self) -> U256 {
        match self {
            OrderOutput::DutchV2(dutch_output) => dutch_output.start_amount,
            OrderOutput::DutchV3(dutch_output_v3) => dutch_output_v3.start_amount,
        }
    }

    pub fn token(&self) -> Address {
        match self {
            OrderOutput::DutchV2(dutch_output) => dutch_output.token,
            OrderOutput::DutchV3(dutch_output_v3) => dutch_output_v3.token,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DutchOutput {
    pub recipient: Address,
    pub start_amount: U256,
    pub end_amount: U256,
    pub token: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DutchOutputV3 {
    token: Address,
    start_amount: U256,
    curve: NonlinearDutchDecay,
    recipient: Address,
    min_amount: U256,
    adjustment_per_gwei_base_fee: U256,
}
