use crate::msg::SudoMsg;
use crate::states::{DELETE_EPOCH, DELETE_OLD_DATA_ENABLED};
use crate::{
    action::sudo::{clean_old_history, clean_up_history, update_account},
    states::UPDATE_ACCOUNT_ENABLED,
};
use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdError, StdResult, Storage};
use elys_bindings::{ElysMsg, ElysQuery};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    mut deps: DepsMut<ElysQuery>,
    env: Env,
    msg: SudoMsg,
    storage: &mut dyn Storage,
) -> StdResult<Response<ElysMsg>> {
    match msg {
        SudoMsg::ClockEndBlock {} => {
            if UPDATE_ACCOUNT_ENABLED.load(deps.storage)? == false {
                return Err(StdError::generic_err("Update account is disabled"));
            }

            let epoch = DELETE_EPOCH.load(deps.storage)?;
            if DELETE_OLD_DATA_ENABLED.load(deps.storage)? == true && env.block.height % epoch == 0
            {
                clean_old_history(&mut deps, epoch)?;
                clean_up_history(&mut deps, env.clone(), epoch)?;
            }

            update_account(deps, env, storage)
        }
    }
}
