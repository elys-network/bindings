use crate::types::StakedAsset;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::DecCoin;

#[cw_serde]
pub struct StakedAssetsResponse {
    pub total_staked_balance: DecCoin,
    pub staked_assets: Vec<StakedAsset>,
}
