use std::cmp::Ordering;

use alloy::primitives::{Address, B256};

#[derive(Debug, Clone)]
pub struct FillEvent {
    pub order_hash: B256,
    pub filler: Address,
    pub swapper: Address,
    pub tx: B256,
    pub block_number: u64,
}

impl FillEvent {
    pub fn new(
        order_hash: B256,
        filler: Address,
        swapper: Address,
        tx: B256,
        block_number: u64,
    ) -> Self {
        Self {
            order_hash,
            filler,
            swapper,
            tx,
            block_number,
        }
    }
}

impl Eq for FillEvent {}

impl PartialEq for FillEvent {
    fn eq(&self, other: &Self) -> bool {
        self.order_hash == other.order_hash
    }
}

impl PartialOrd for FillEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FillEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.block_number.cmp(&other.block_number)
    }
}
