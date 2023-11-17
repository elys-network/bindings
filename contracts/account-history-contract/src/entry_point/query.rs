use crate::states::HISTORY;

use super::*;
use msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        UserHistory { user_address } => to_json_binary(&HISTORY.load(deps.storage, &user_address)?),
    }
}
