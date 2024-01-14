use cosmwasm_schema::cw_serde;

use super::TotalValueOfAssetResp;

#[cw_serde]
pub struct TotalValuePerAssetResp {
    pub list_asset_value: Vec<TotalValueOfAssetResp>,
}
