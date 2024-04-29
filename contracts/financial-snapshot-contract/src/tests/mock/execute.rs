use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use elys_bindings::{ElysMsg, ElysQuery};

pub fn execute(
    _deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response<ElysMsg>> {
    Ok(Response::default())
}
