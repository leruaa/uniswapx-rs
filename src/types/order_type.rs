use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum OrderType {
    Dutch,
    #[serde(rename = "Dutch_V2")]
    DutchV2,
    #[serde(rename = "Dutch_V3")]
    DutchV3,
    Limit,
    Relay,
    #[default]
    #[serde(rename = "Dutch_V1_V2")]
    DutchV1V2,
    Priority,
}
