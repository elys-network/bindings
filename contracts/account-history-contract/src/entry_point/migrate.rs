use cosmwasm_std::Empty;
use cw_utils::DAY;
use elys_bindings::{ElysMsg, ElysQuery};

use crate::states::INTERVAL;
use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<ElysQuery>, _env: Env, _msg: Empty) -> StdResult<Response<ElysMsg>> {
    INTERVAL.save(deps.storage, &DAY)?;
    Ok(Response::new())
}
