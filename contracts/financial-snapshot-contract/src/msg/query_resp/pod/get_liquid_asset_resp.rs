use crate::types::LiquidAsset;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetLiquidAssetResp {
    pub liquid_asset: LiquidAsset,
}
