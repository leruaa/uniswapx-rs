use alloy_primitives::{address, Address};

pub struct ReactorConfig {
    pub address: Address,
}

impl ReactorConfig {
    pub fn new(chain_id: u64) -> Self {
        match chain_id {
            1 => Self {
                address: address!("6000da47483062A0D734Ba3dc7576Ce6A0B645C4"),
            },
            chain_id => unimplemented!("Chain {chain_id} not supported"),
        }
    }
}
