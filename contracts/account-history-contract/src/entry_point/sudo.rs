use crate::action::sudo::update_account;
use crate::msg::SudoMsg;
use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdResult};
use elys_bindings::{ElysMsg, ElysQuery};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<ElysQuery>, env: Env, msg: SudoMsg) -> StdResult<Response<ElysMsg>> {
    match msg {
        SudoMsg::ClockEndBlock {} => update_account(deps, env, true),
    }
}
