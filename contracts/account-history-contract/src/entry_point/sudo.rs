use crate::msg::SudoMsg;
use crate::{action::sudo::update_account, states::UPDATE_ACCOUNT_ENABLED};
use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdError, StdResult};
use elys_bindings::{ElysMsg, ElysQuery};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<ElysQuery>, env: Env, msg: SudoMsg) -> StdResult<Response<ElysMsg>> {
    match msg {
        SudoMsg::ClockEndBlock {} => {
            if UPDATE_ACCOUNT_ENABLED.load(deps.storage)? == false {
                return Err(StdError::generic_err("Update account is disabled"));
            }
            update_account(deps, env)
        }
    }
}
