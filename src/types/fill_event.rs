use std::cmp::Ordering;

use ethers::types::{Address, Bytes, H256, U64};

#[derive(Debug, Clone)]
pub struct FillEvent {
    pub order_hash: Bytes,
    pub filler: Address,
    pub swapper: Address,
    pub tx: H256,
    pub block_number: U64,
}

impl FillEvent {
    pub fn new(
        order_hash: Bytes,
        filler: Address,
        swapper: Address,
        tx: H256,
        block_number: U64,
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
        self.block_number.partial_cmp(&other.block_number)
    }
}

impl Ord for FillEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.block_number.cmp(&other.block_number)
    }
}
