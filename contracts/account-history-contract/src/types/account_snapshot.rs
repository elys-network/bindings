use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal, Decimal256, Uint128};
use cw_utils::Expiration;

use super::{LiquidAsset, Portfolio, Reward, StakedAsset, TotalBalance};

#[cw_serde]
pub struct AccountSnapshot {
    pub date: Expiration,
    pub total_balance: TotalBalance,
    pub portfolio: Portfolio,
    pub reward: Reward,
    pub liquid_asset: LiquidAsset,
    pub staked_assets: Vec<StakedAsset>,
}

impl AccountSnapshot {
    pub fn zero(value_denom: &String) -> Self {
        Self {
            date: Expiration::Never {},
            total_balance: TotalBalance {
                total_balance: DecCoin::new(Decimal256::zero(), value_denom),
                portfolio_usd: DecCoin::new(Decimal256::zero(), value_denom),
                reward_usd: DecCoin::new(Decimal256::zero(), value_denom),
            },
            portfolio: Portfolio {
                balance_usd: DecCoin::new(Decimal256::zero(), value_denom),
                liquid_assets_usd: DecCoin::new(Decimal256::zero(), value_denom),
                staked_committed_usd: DecCoin::new(Decimal256::zero(), value_denom),
                liquidity_positions_usd: DecCoin::new(Decimal256::zero(), value_denom),
                leverage_lp_usd: DecCoin::new(Decimal256::zero(), value_denom),
                margin_usd: DecCoin::new(Decimal256::zero(), value_denom),
                usdc_earn_usd: DecCoin::new(Decimal256::zero(), value_denom),
                borrows_usd: DecCoin::new(Decimal256::zero(), value_denom),
            },
            reward: Reward {
                usdc_usd: Decimal::zero(),
                eden_usd: Decimal::zero(),
                eden_boost: Uint128::zero(),
                other_usd: Decimal::zero(),
                total_usd: Decimal::zero(),
            },
            liquid_asset: LiquidAsset {
                total_liquid_asset_balance: DecCoin::new(Decimal256::zero(), value_denom),
                total_available_balance: DecCoin::new(Decimal256::zero(), value_denom),
                total_in_orders_balance: DecCoin::new(Decimal256::zero(), value_denom),
                available_asset_balance: vec![],
                in_orders_asset_balance: vec![],
                total_value_per_asset: vec![],
            },
            staked_assets: vec![],
        }
    }
}
