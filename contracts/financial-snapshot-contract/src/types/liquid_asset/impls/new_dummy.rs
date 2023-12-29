use crate::types::liquid_asset::liquid_asset::LiquidAsset;
use cosmwasm_std::{Decimal, Uint128};

impl LiquidAsset {
    pub fn new_dummy() -> LiquidAsset {
        LiquidAsset {
            asset: "atom".to_string(),
            change_percent_24hr: Decimal::from_atomics(Uint128::new(52), 1).unwrap(),
            total_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
            total_token: 100,
            available_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
            available_token: 0,
            in_order_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            in_order_token: 0,
        }
    }

    pub fn new_dummys() -> Vec<LiquidAsset> {
        vec![LiquidAsset {
            asset: "atom".to_string(),
            change_percent_24hr: Decimal::from_atomics(Uint128::new(51), 1).unwrap(),
            total_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
            total_token: 100,
            available_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
            available_token: 0,
            in_order_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            in_order_token: 0,
        }]
    }
}
