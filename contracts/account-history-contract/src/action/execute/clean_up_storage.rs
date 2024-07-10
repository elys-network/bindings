use cosmwasm_std::{DepsMut, Response, StdResult};

use crate::states::{HISTORY, OLD_HISTORY_2, USER_ADDRESS_QUEUE};
use elys_bindings::{ElysMsg, ElysQuery};

pub fn clean_up_storage(deps: &mut DepsMut<ElysQuery>, limit: u64) -> StdResult<Response<ElysMsg>> {
    // Delete history values
    for _ in 0..limit {
        if let Some(val) = HISTORY.first(deps.storage)? {
            HISTORY.remove(deps.storage, &val.0);
        }
        if let Some(val) = OLD_HISTORY_2.first(deps.storage)? {
            OLD_HISTORY_2.remove(deps.storage, &val.0);
        }
        if USER_ADDRESS_QUEUE.front(deps.storage).is_ok() {
            USER_ADDRESS_QUEUE.pop_front(deps.storage)?;
        }
    }
    Ok(Response::default())
}
