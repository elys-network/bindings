use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal256;

use crate::account_history::msg::query_resp::StakeAssetBalanceBreakdown;

#[cw_serde]
pub struct Portfolio {
    pub balance_usd: Decimal256,
    pub liquid_assets_usd: Decimal256,
    pub staked_committed_usd: Decimal256,
    pub liquidity_positions_usd: Decimal256,
    pub leverage_lp_usd: Decimal256,
    pub perpetual_assets_usd: Decimal256,
    pub usdc_earn_usd: Decimal256,
    pub borrows_usd: Decimal256,
    pub stake_balance_breakdown: StakeAssetBalanceBreakdown
}

// implement zero
impl Portfolio {
    pub fn zero(_value_denom: &String) -> Self {
        Self {
            balance_usd: Decimal256::zero(),
            liquid_assets_usd: Decimal256::zero(),
            staked_committed_usd: Decimal256::zero(),
            liquidity_positions_usd: Decimal256::zero(),
            leverage_lp_usd: Decimal256::zero(),
            perpetual_assets_usd: Decimal256::zero(),
            usdc_earn_usd: Decimal256::zero(),
            borrows_usd: Decimal256::zero(),
            stake_balance_breakdown: StakeAssetBalanceBreakdown::default(),
        }
    }
}

// implement default
impl Default for Portfolio {
    fn default() -> Self {
        Self::zero(&"usdc".to_string())
    }
}
