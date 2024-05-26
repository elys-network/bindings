use cosmwasm_std::StdError;
use elys_bindings::trade_shield::states::PROCESS_ORDERS_ENABLED;

use super::*;
use crate::action::sudo::*;
use crate::msg::SudoMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    deps: DepsMut<ElysQuery>,
    env: Env,
    msg: SudoMsg,
) -> Result<Response<ElysMsg>, ContractError> {
    match msg {
        SudoMsg::ClockEndBlock {} => {
            if PROCESS_ORDERS_ENABLED.load(deps.storage)? == false {
                return Err(StdError::generic_err("process order is disable").into());
            }
            process_orders(deps, env)
        }
    }
}
