use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

use crate::types::StakedAsset;

#[cw_serde]
pub struct StakedAssetResponse {
    pub staked_assets: Vec<StakedAsset>,
    pub total_balance: Decimal,
}
