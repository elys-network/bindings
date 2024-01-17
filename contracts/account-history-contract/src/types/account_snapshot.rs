use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal};
use cw_utils::Expiration;
use elys_bindings::types::EarnType;

use super::CoinValue;

#[cw_serde]
pub struct StakedAssetResponse {
    pub staked_assets: Vec<StakedAsset>,
    pub total_balance: Decimal,
}

#[cw_serde]
pub struct StakedAsset {
    pub program: EarnType,
    pub apr: Decimal,
    pub available: Decimal,
    pub staked: Decimal,
    pub rewards : Decimal,
}

#[cw_serde]
pub struct AccountSnapshot {
    pub date: Expiration,
    pub total_liquid_asset_balance: DecCoin,
    pub total_available_balance: DecCoin,
    pub total_in_orders_balance: DecCoin,
    pub available_asset_balance: Vec<CoinValue>,
    pub in_orders_asset_balance: Vec<CoinValue>,
    pub total_value_per_asset: Vec<CoinValue>,

    // staked asset
    pub total_staked_asset_balance: Decimal,
    pub staked_assets: Vec<StakedAsset>,
}
