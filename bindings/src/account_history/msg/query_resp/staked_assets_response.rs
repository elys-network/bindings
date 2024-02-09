use cosmwasm_schema::cw_serde;
use cosmwasm_std::DecCoin;

use crate::account_history::types::StakedAssets;

#[cw_serde]
pub struct StakedAssetsResponse {
    pub total_staked_balance: DecCoin,
    pub staked_assets: StakedAssets,
}
