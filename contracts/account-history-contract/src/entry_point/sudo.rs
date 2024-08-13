use crate::action::execute::clean_up_storage;
use crate::states::DELETE_OLD_DATA_ENABLED;
use crate::{msg::SudoMsg, states::DELETE_EPOCH};
use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdResult};
use elys_bindings::{ElysMsg, ElysQuery};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(mut deps: DepsMut<ElysQuery>, _env: Env, msg: SudoMsg) -> StdResult<Response<ElysMsg>> {
    match msg {
        SudoMsg::ClockEndBlock {} => {
            let epoch = DELETE_EPOCH.load(deps.storage)?;
            if DELETE_OLD_DATA_ENABLED.load(deps.storage)? == true {
                clean_up_storage(&mut deps, epoch)?;
            }
            Ok(Response::new())
        }
    }
}
