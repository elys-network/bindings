use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::pod::GetLiquidAssetResp};

pub fn get_pod_liquid_asset(deps: Deps<ElysQuery>, asset: String) -> Result<GetLiquidAssetResp, ContractError> {
    let liquid_assets: Vec<LiquidAsset> = LIQUID_ASSETS.load(deps.storage)?;
    let have_assets: Option<&LiquidAsset> = liquid_assets.iter().find(|liquid_asset| liquid_asset.asset == asset);
    let resp: GetLiquidAssetResp = GetLiquidAssetResp {
        liquid_asset: match have_assets {
            Some(liquid_asset) => liquid_asset.to_owned(),
            None => LiquidAsset::new_dummy(),
        },
    };

    Ok(resp)
}
