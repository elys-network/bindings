use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal256};

#[cw_serde]
pub struct Portfolio {
    pub balance_usd: DecCoin,
    pub liquid_assets_usd: DecCoin,
    pub staked_committed_usd: DecCoin,
    pub liquidity_positions_usd: DecCoin,
    pub leverage_lp_usd: DecCoin,
    pub perpetual_assets_usd: DecCoin,
    pub usdc_earn_usd: DecCoin,
    pub borrows_usd: DecCoin,
}

// implement zero
impl Portfolio {
    pub fn zero(value_denom: &String) -> Self {
        Self {
            balance_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            liquid_assets_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            staked_committed_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            liquidity_positions_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            leverage_lp_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            perpetual_assets_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            usdc_earn_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            borrows_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
        }
    }
}

// implement default
impl Default for Portfolio {
    fn default() -> Self {
        Self::zero(&"usdc".to_string())
    }
}
