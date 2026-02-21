use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

use crate::types::order::NonlinearDutchDecay;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderInput {
    DutchV2(DutchInput),
    DutchV3(DutchInputV3),
}

impl Default for OrderInput {
    fn default() -> Self {
        OrderInput::DutchV2(DutchInput::default())
    }
}

impl OrderInput {
    pub fn start_amount(&self) -> U256 {
        match self {
            OrderInput::DutchV2(input) => input.start_amount,
            OrderInput::DutchV3(input) => input.start_amount,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DutchInput {
    pub start_amount: U256,
    pub end_amount: U256,
    pub token: Address,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DutchInputV3 {
    pub token: Address,
    pub start_amount: U256,
    pub curve: NonlinearDutchDecay,
    pub max_amount: U256,
    pub adjustment_per_gwei_base_fee: U256,
}

#[cfg(test)]
mod tests {
    use crate::types::order_input::DutchInputV3;

    #[test]
    fn deserialize_dutch_v3_order_input() {
        let data = r#"
        {
           "startAmount":"9127972",
           "maxAmount":"9127972",
           "curve":{
              "relativeAmounts":[
                 "0"
              ],
              "relativeBlocks":[
                 32
              ]
           },
           "token":"0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f",
           "adjustmentPerGweiBaseFee":"0"
        }
        "#;

        let _ = serde_json::from_str::<DutchInputV3>(data).unwrap();
    }
}
