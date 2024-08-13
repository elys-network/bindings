use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal256};

use crate::trade_shield::types::CoinValue;

#[cw_serde]
pub struct LiquidAsset {
    pub total_liquid_asset_balance: DecCoin,
    pub total_available_balance: DecCoin,
    pub total_in_orders_balance: DecCoin,
    pub available_asset_balance: Vec<CoinValue>,
    pub in_orders_asset_balance: Vec<CoinValue>,
    pub total_value_per_asset: Vec<CoinValue>,
}

// implement zero
impl LiquidAsset {
    pub fn zero(value_denom: &String) -> Self {
        Self {
            total_liquid_asset_balance: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            total_available_balance: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            total_in_orders_balance: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            available_asset_balance: vec![],
            in_orders_asset_balance: vec![],
            total_value_per_asset: vec![],
        }
    }
}

// implement default
impl Default for LiquidAsset {
    fn default() -> Self {
        Self::zero(&"usdc".to_string())
    }
}
