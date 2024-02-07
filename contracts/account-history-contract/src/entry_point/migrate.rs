use cosmwasm_std::Empty;
use cw_utils::DAY;

use crate::states::INTERVAL;

use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<ElysQuery>, _env: Env, _msg: Empty) -> StdResult<Response<ElysMsg>> {
    INTERVAL.save(deps.storage, &DAY)?;
    Ok(Response::new())
}
