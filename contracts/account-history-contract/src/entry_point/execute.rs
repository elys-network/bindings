use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use elys_bindings::{account_history::msg::ExecuteMsg, ElysMsg, ElysQuery};

use crate::{
    action::{
        execute::{add_user_address_to_queue, clean_up_storage},
        sudo::update_account_chain,
    },
    states::{
        DELETE_EPOCH, DELETE_OLD_DATA_ENABLED, PARAMS_ADMIN, PROCESSED_ACCOUNT_PER_BLOCK,
        TRADE_SHIELD_ADDRESS, UPDATE_ACCOUNT_ENABLED,
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
            delete_old_data_enabled,
            delete_epoch,
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

            if let Some(delete_old_data_enabled) = delete_old_data_enabled {
                DELETE_OLD_DATA_ENABLED.save(deps.storage, &delete_old_data_enabled)?;
            }

            if let Some(delete_epoch) = delete_epoch {
                DELETE_EPOCH.save(deps.storage, &delete_epoch)?;
            }
            Ok(Response::new())
        }
        ExecuteMsg::UpdateAccount {} => {
            if info.sender != PARAMS_ADMIN.load(deps.storage)? {
                return Err(StdError::generic_err("Unauthorized"));
            }
            let resp = update_account_chain(deps, env)?;
            Ok(resp)
        }
        ExecuteMsg::CleanStorage { limit } => {
            if info.sender != PARAMS_ADMIN.load(deps.storage)? {
                return Err(StdError::generic_err("Unauthorized"));
            }
            let resp = clean_up_storage(&mut deps, limit)?;
            Ok(resp)
        }
    }
}
