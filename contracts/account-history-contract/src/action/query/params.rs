use crate::{
    action::{EXPIRATION, PAGINATION, TRADE_SHIELD_ADDRESS, VALUE_DENOM},
    msg::query_resp::ParamsResp,
};
use cosmwasm_std::{Deps, StdResult};
use elys_bindings::ElysQuery;

pub fn params(deps: Deps<ElysQuery>) -> StdResult<ParamsResp> {
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let expiration = EXPIRATION.load(deps.storage)?;
    let pagination = PAGINATION.load(deps.storage)?;
    let trade_shield_address = TRADE_SHIELD_ADDRESS.load(deps.storage)?;

    Ok(ParamsResp {
        expiration,
        pagination,
        value_denom,
        trade_shield_address,
    })
}
