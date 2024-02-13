use cosmwasm_schema::cw_serde;
use cw_utils::Expiration;

use super::{
    LiquidAsset, PerpetualAssets, PoolBalances, Portfolio, Reward, StakedAssets, TotalBalance,
};

#[cw_serde]
pub struct AccountSnapshot {
    pub date: Expiration,
    pub total_balance: TotalBalance,
    pub portfolio: Portfolio,
    pub reward: Reward,
    pub pool_balances: PoolBalances,
    pub liquid_asset: LiquidAsset,
    pub staked_assets: StakedAssets,
    pub perpetual_assets: PerpetualAssets,
}

impl AccountSnapshot {
    pub fn zero(value_denom: &String) -> Self {
        Self {
            date: Expiration::Never {},
            total_balance: TotalBalance::zero(value_denom),
            portfolio: Portfolio::zero(value_denom),
            reward: Reward::default(),
            pool_balances: PoolBalances::default(),
            liquid_asset: LiquidAsset::zero(value_denom),
            staked_assets: StakedAssets::default(),
            perpetual_assets: PerpetualAssets::default(),
        }
    }
}

// implement default
impl Default for AccountSnapshot {
    fn default() -> Self {
        Self {
            date: Expiration::Never {},
            total_balance: TotalBalance::default(),
            portfolio: Portfolio::default(),
            reward: Reward::default(),
            pool_balances: PoolBalances::default(),
            liquid_asset: LiquidAsset::default(),
            staked_assets: StakedAssets::default(),
            perpetual_assets: PerpetualAssets::default(),
        }
    }
}
