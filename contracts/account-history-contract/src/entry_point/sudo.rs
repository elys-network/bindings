use super::*;
use crate::action::sudo::*;
use crate::msg::SudoMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<ElysQuery>, env: Env, msg: SudoMsg) -> StdResult<Response<ElysMsg>> {
    match msg {
        SudoMsg::UpdateAccounts {} => update_account(deps, env),
    }
}
