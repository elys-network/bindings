use cosmwasm_schema::cw_serde;
use cosmwasm_std::DecCoin;

use super::TotalValueOfAssetResp;

#[cw_serde]
pub struct GetLiquidAssetsResp {
    pub liquid_assets: Vec<TotalValueOfAssetResp>,
    pub total_liquid_asset_balance: DecCoin,
}
