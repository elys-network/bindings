use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::pod::GetLiquidAssetsResp};

pub fn get_pod_liquid_assets(deps: Deps<ElysQuery>) -> Result<GetLiquidAssetsResp, ContractError> {
    let liquid_assets: Vec<LiquidAsset> = LIQUID_ASSETS.load(deps.storage)?;
    let resp: GetLiquidAssetsResp;

    if liquid_assets.len() > 0 {
        resp = GetLiquidAssetsResp {
            liquid_assets: liquid_assets
        };
    } else {
        resp = GetLiquidAssetsResp {
            liquid_assets: LiquidAsset::new_dummys()
        };
    }

    Ok(resp)
}
