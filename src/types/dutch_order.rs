use alloy::{
    primitives::{Address, Bytes, U256},
    sol,
    sol_types::SolValue,
};
use anyhow::Result;

sol! {
    #[derive(Debug)]
    struct OrderInfo {
        address reactor;
        address swapper;
        uint256 nonce;
        uint256 deadline;
        address additionalValidationContract;
        bytes additionalValidationData;
    }

    #[derive(Debug)]
    struct DutchOutput {
        address token;
        uint256 startAmount;
        uint256 endAmount;
        address recipient;
    }

    #[derive(Debug)]
    struct DutchInput {
        address token;
        uint256 startAmount;
        uint256 endAmount;
    }

    #[derive(Debug)]
    struct DutchOrderV1 {
        OrderInfo info;
        uint256 decayStartTime;
        uint256 decayEndTime;
        address exclusiveFiller;
        uint256 exclusivityOverrideBps;
        DutchInput input;
        DutchOutput[] outputs;
    }

    #[derive(Debug)]
    struct CosignerData {
        uint256 decayStartTime;
        uint256 decayEndTime;
        address exclusiveFiller;
        uint256 exclusivityOverrideBps;
        uint256 inputOverride;
        uint256[] outputOverrides;
    }

    #[derive(Debug)]
    struct DutchOrderV2 {
        OrderInfo info;
        address cosigner;
        DutchInput input;
        DutchOutput[] outputs;
        CosignerData cosignerData;
        bytes cosignature;
    }
}

pub enum DutchOrder {
    V1(DutchOrderV1),
    V2(DutchOrderV2),
}

impl DutchOrder {
    pub fn try_from_v1(encoded: &Bytes) -> Result<Self> {
        Ok(Self::V1(DutchOrderV1::abi_decode(encoded, true)?))
    }

    pub fn try_from_v2(encoded: &Bytes) -> Result<Self> {
        Ok(Self::V2(DutchOrderV2::abi_decode(encoded, true)?))
    }

    pub fn deadline(&self) -> U256 {
        match self {
            DutchOrder::V1(order) => order.info.deadline,
            DutchOrder::V2(order) => order.info.deadline,
        }
    }

    pub fn input_token(&self) -> Address {
        match self {
            DutchOrder::V1(order) => order.input.token,
            DutchOrder::V2(order) => order.input.token,
        }
    }

    pub fn decay_start_time(&self) -> U256 {
        match self {
            DutchOrder::V1(order) => order.decayStartTime,
            DutchOrder::V2(order) => order.cosignerData.decayStartTime,
        }
    }

    pub fn decay_end_time(&self) -> U256 {
        match self {
            DutchOrder::V1(order) => order.decayEndTime,
            DutchOrder::V2(order) => order.cosignerData.decayEndTime,
        }
    }

    pub fn input(&self) -> &DutchInput {
        match self {
            DutchOrder::V1(order) => &order.input,
            DutchOrder::V2(order) => &order.input,
        }
    }

    pub fn outputs(&self) -> &Vec<DutchOutput> {
        match self {
            DutchOrder::V1(order) => &order.outputs,
            DutchOrder::V2(order) => &order.outputs,
        }
    }

    pub fn exclusive_filler(&self) -> Address {
        match self {
            DutchOrder::V1(order) => order.exclusiveFiller,
            DutchOrder::V2(order) => order.cosignerData.exclusiveFiller,
        }
    }

    pub fn exclusivity_override_bps(&self) -> U256 {
        match self {
            DutchOrder::V1(order) => order.exclusivityOverrideBps,
            DutchOrder::V2(order) => order.cosignerData.exclusivityOverrideBps,
        }
    }

    pub fn resolve(&self, timestamp: u64) -> OrderResolution {
        let timestamp = U256::from(timestamp);

        if self.deadline().lt(&timestamp) {
            return OrderResolution::Expired;
        };

        // resolve over the decay curve

        let input: ResolvedInput = ResolvedInput {
            token: self.input_token(),
            amount: resolve_decay(
                timestamp,
                self.decay_start_time(),
                self.decay_end_time(),
                self.input().startAmount,
                self.input().endAmount,
            ),
        };

        let outputs = self
            .outputs()
            .iter()
            .map(|output| {
                let mut amount = resolve_decay(
                    timestamp,
                    self.decay_start_time(),
                    self.decay_end_time(),
                    output.startAmount,
                    output.endAmount,
                );

                // add exclusivity override to amount
                if self.decay_start_time().gt(&timestamp) && !self.exclusive_filler().is_zero() {
                    let exclusivity = self
                        .exclusivity_override_bps()
                        .wrapping_add(U256::from(10000));
                    let exclusivity = exclusivity.wrapping_mul(amount);
                    amount = exclusivity.wrapping_div(U256::from(10000));
                };

                ResolvedOutput {
                    token: output.token,
                    amount,
                    recipient: output.recipient,
                }
            })
            .collect();

        OrderResolution::Resolved(ResolvedOrder { input, outputs })
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedInput {
    pub token: Address,
    pub amount: U256,
}

#[derive(Debug, Clone)]
pub struct ResolvedOutput {
    pub token: Address,
    pub amount: U256,
    pub recipient: Address,
}

#[derive(Debug, Clone)]
pub struct ResolvedOrder {
    pub input: ResolvedInput,
    pub outputs: Vec<ResolvedOutput>,
}

#[derive(Debug)]
pub enum OrderResolution {
    Resolved(ResolvedOrder),
    Expired,
    Invalid,
}

fn resolve_decay(
    at_time: U256,
    start_time: U256,
    end_time: U256,
    start_amount: U256,
    end_amount: U256,
) -> U256 {
    if end_time.le(&at_time) {
        return end_amount;
    }

    if at_time.le(&start_time) {
        return start_amount;
    }

    if end_time.eq(&start_time) {
        return start_amount;
    }

    if start_amount.eq(&end_amount) {
        return start_amount;
    }

    let duration = end_time.wrapping_sub(start_time);
    let elapsed = at_time.wrapping_sub(start_time);
    // TODO: better handle overflows
    if start_amount.gt(&end_amount) {
        // decaying downward
        let decay = start_amount
            .wrapping_sub(end_amount)
            .wrapping_mul(elapsed)
            .wrapping_div(duration);
        start_amount.wrapping_sub(decay)
    } else {
        // decaying upward
        let decay = end_amount
            .wrapping_sub(start_amount)
            .wrapping_mul(elapsed)
            .wrapping_div(duration);
        start_amount.wrapping_add(decay)
    }
}
