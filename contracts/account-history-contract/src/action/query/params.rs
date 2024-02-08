use crate::{
    msg::query_resp::ParamsResp,
    states::{EXPIRATION, PAGINATION, TRADE_SHIELD_ADDRESS},
};
use cosmwasm_std::{Deps, StdResult};
use elys_bindings::ElysQuery;

pub fn params(deps: Deps<ElysQuery>) -> StdResult<ParamsResp> {
    let expiration = EXPIRATION.load(deps.storage)?;
    let pagination = PAGINATION.load(deps.storage)?;
    let trade_shield_address = TRADE_SHIELD_ADDRESS.load(deps.storage)?;

    Ok(ParamsResp {
        expiration,
        pagination,
        trade_shield_address,
    })
}
