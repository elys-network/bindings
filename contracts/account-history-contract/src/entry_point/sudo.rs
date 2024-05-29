use crate::msg::SudoMsg;
use crate::states::DELETE_OLD_DATA_ENABLED;
use crate::{
    action::sudo::{clean_old_history, clean_up_history, update_account},
    states::UPDATE_ACCOUNT_ENABLED,
};
use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdError, StdResult};
use elys_bindings::{ElysMsg, ElysQuery};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(mut deps: DepsMut<ElysQuery>, env: Env, msg: SudoMsg) -> StdResult<Response<ElysMsg>> {
    match msg {
        SudoMsg::ClockEndBlock {} => {
            if UPDATE_ACCOUNT_ENABLED.load(deps.storage)? == false {
                return Err(StdError::generic_err("Update account is disabled"));
            }
            if DELETE_OLD_DATA_ENABLED.load(deps.storage)? == true {
                clean_old_history(&mut deps, 1000u64)?;
                clean_up_history(&mut deps, env.clone(), 1000u64)?;
            }

            update_account(deps, env)
        }
    }
}
