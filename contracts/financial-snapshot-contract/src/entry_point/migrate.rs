use cosmwasm_std::Empty;
use cosmwasm_std::{
    entry_point,
};

use super::*;
use elys_bindings::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut<ElysQuery>, _env: Env, _msg: Empty) -> StdResult<Response<ElysMsg>> {
    Ok(Response::new())
}