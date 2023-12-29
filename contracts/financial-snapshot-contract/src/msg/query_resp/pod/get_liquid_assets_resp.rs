use crate::types::LiquidAsset;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetLiquidAssetsResp {
    pub liquid_assets: Vec<LiquidAsset>,
}
