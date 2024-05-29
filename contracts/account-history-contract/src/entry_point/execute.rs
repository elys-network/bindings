use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use elys_bindings::{account_history::msg::ExecuteMsg, ElysMsg, ElysQuery};

use crate::{
    action::{
        execute::add_user_address_to_queue,
        sudo::{clean_old_history, clean_up_history, update_account},
    },
    states::{
        PARAMS_ADMIN, PROCESSED_ACCOUNT_PER_BLOCK, TRADE_SHIELD_ADDRESS, UPDATE_ACCOUNT_ENABLED,
    },
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    mut deps: DepsMut<ElysQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<ElysMsg>> {
    match msg {
        ExecuteMsg::AddUserAddressToQueue { user_address } => {
            let trade_shield_address = match TRADE_SHIELD_ADDRESS.load(deps.storage)? {
                Some(addr) => addr,
                None => return Err(StdError::generic_err("Unauthorized")),
            };
            if trade_shield_address.as_str() != info.sender.as_str() {
                return Err(StdError::generic_err("Unauthorized"));
            }
            add_user_address_to_queue(deps, user_address)?;
            Ok(Response::new())
        }
        ExecuteMsg::ChangeParams {
            update_account_enabled,
            processed_account_per_block,
        } => {
            let params_admin = PARAMS_ADMIN.load(deps.storage)?;

            if params_admin.as_str() != info.sender.as_str() {
                return Err(StdError::generic_err("Unauthorized"));
            }

            if let Some(processed_account_per_block) = processed_account_per_block {
                PROCESSED_ACCOUNT_PER_BLOCK.save(deps.storage, &processed_account_per_block)?;
            }

            if let Some(update_account_enabled) = update_account_enabled {
                UPDATE_ACCOUNT_ENABLED.save(deps.storage, &update_account_enabled)?;
            }
            Ok(Response::new())
        }
        ExecuteMsg::UpdateAccount {} => {
            if info.sender != PARAMS_ADMIN.load(deps.storage)? {
                return Err(StdError::generic_err("Unauthorized"));
            }
            let resp = update_account(deps, env)?;
            Ok(resp)
        }
        ExecuteMsg::CleanHistory { limit } => {
            if info.sender != PARAMS_ADMIN.load(deps.storage)? {
                return Err(StdError::generic_err("Unauthorized"));
            }
            let resp = clean_up_history(&mut deps, env, limit)?;
            Ok(resp)
        }
        ExecuteMsg::CleanOldHistory { limit } => {
            if info.sender != PARAMS_ADMIN.load(deps.storage)? {
                return Err(StdError::generic_err("Unauthorized"));
            }
            let resp = clean_old_history(&mut deps, limit)?;
            Ok(resp)
        }
    }
}
