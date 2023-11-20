use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum OrderType {
    #[default]
    Dutch,
    DutchLimit,
}
