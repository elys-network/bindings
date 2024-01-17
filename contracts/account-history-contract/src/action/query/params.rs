use crate::{
    action::{EXPIRATION, PAGINATION, VALUE_DENOM},
    msg::query_resp::ParamsResp,
};
use cosmwasm_std::{Deps, StdResult};
use elys_bindings::ElysQuery;

pub fn params(deps: Deps<ElysQuery>) -> StdResult<ParamsResp> {
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let expiration = EXPIRATION.load(deps.storage)?;
    let pagination = PAGINATION.load(deps.storage)?;

    Ok(ParamsResp {
        expiration,
        pagination,
        value_denom,
    })
}
