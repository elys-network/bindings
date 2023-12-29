use super::*;
use cosmwasm_std::{Decimal, Uint128};
use crate::bindings::{query::ElysQuery, querier::ElysQuerier};
use crate::types::ElysDenom;
use crate::msg::query_resp::earn::GetUsdcPriceResp;

pub fn get_usdc_price(deps: Deps<ElysQuery>) -> Result<GetUsdcPriceResp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);
           
    let usdc_denom_entry = querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
    let usdc_display_denom = usdc_denom_entry.entry.display_name;
    
    let usdc_oracle_price = querier.get_oracle_price(usdc_display_denom.clone(), ElysDenom::AnySource.as_str().to_string(), 0)?;
    let usdc_usd_price = usdc_oracle_price.price.price.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();
    let resp = GetUsdcPriceResp {
        price: usdc_usd_price,
    };
    Ok(resp)
}