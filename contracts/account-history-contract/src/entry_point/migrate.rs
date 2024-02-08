use cosmwasm_std::Empty;
use elys_bindings::{ElysMsg, ElysQuery};

use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut<ElysQuery>, _env: Env, _msg: Empty) -> StdResult<Response<ElysMsg>> {
    Ok(Response::new())
}
