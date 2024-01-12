use crate::types::reward::reward::Reward;
use cosmwasm_std::{Decimal, Uint128};

impl Reward {
    pub fn new_dummy() -> Reward {
        Reward {
            usdc_usd: Decimal::zero(),
            eden_usd: Decimal::zero(),
            eden_boost: Uint128::zero(),
            other_usd: Decimal::zero(),
            total_usd: Decimal::zero(),
        }
    }
}
