use cosmwasm_schema::cw_serde;
use cosmwasm_std::DecCoin;

use super::LiquidAsset;

#[cw_serde]
pub struct GetLiquidAssetsResp {
    pub liquid_assets: Vec<LiquidAsset>,
    pub total_liquid_asset_balance: DecCoin,
}
