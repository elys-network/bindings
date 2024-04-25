use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use elys_bindings::{account_history::msg::ExecuteMsg, ElysMsg, ElysQuery};

use crate::{action::execute::add_wallet, states::TRADE_SHIELD_ADDRESS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<ElysMsg>> {
    match msg {
        ExecuteMsg::AddAddress { wallet } => {
            let trade_shield_address = match TRADE_SHIELD_ADDRESS.load(deps.storage)? {
                Some(addr) => addr,
                None => return Err(StdError::generic_err("Unauthorized")),
            };
            if trade_shield_address.as_str() != info.sender.as_str() {
                return Err(StdError::generic_err("Unauthorized"));
            }
            add_wallet(deps.storage, wallet)?;
            Ok(Response::new())
        }
    }
}
