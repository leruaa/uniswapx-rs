use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OrderStatus {
    #[default]
    Open,
    Filled,
    Cancelled,
    Expired,
    Error,
    InsufficientFunds,
}
