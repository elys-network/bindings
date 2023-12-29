use crate::types::liquidity_position::liquidity_position::LiquidityPosition;
use cosmwasm_std::{Decimal, Uint128};

impl LiquidityPosition {
    pub fn init() -> LiquidityPosition {
        LiquidityPosition {
            pool_id: 0,
            pool_rate_percent: vec![],
            apr_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            inflationary_eden_rewards: 0,
            external_rewards_apr_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            fee_apr_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            fees_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            current_tvl_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            balance_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            rewards_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
        }
    }
}
