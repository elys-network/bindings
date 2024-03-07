use super::*;
use crate::msg::query_resp::GetAssetPriceResp;
use cosmwasm_std::Decimal;

pub fn get_asset_price(
    deps: Deps<ElysQuery>,
    denom: String,
) -> Result<GetAssetPriceResp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let usd_price: Decimal = querier.get_asset_price(denom)?;

    let resp: GetAssetPriceResp = GetAssetPriceResp { usd_price };

    Ok(resp)
}
