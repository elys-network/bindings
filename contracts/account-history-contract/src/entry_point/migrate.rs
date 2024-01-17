use cosmwasm_std::{Empty, StdError};

use super::*;
use crate::states::HISTORY;


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<ElysQuery>, _env: Env, _msg: Empty) -> StdResult<Response<ElysMsg>> {
    let keys: Vec<String> = HISTORY
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|key_result| key_result.map(|key| key))
        .collect::<Result<Vec<String>, StdError>>()?;

    for key in keys {
        HISTORY.remove(deps.storage, key.as_str());
    }
    Ok(Response::new())
}