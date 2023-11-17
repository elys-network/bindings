use elys_bindings::types::PageRequest;

use super::*;
use crate::msg::InstantiateMsg;
use crate::states::{AMM_ROUTES, EXPIRATION, PAGINATION};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<ElysMsg>> {
    EXPIRATION.save(deps.storage, &msg.expiration)?;
    PAGINATION.save(
        deps.storage,
        &PageRequest {
            key: None,
            limit: msg.limit,
            reverse: false,
            offset: None,
            count_total: false,
        },
    )?;
    AMM_ROUTES.save(deps.storage, &msg.amm_routes)?;
    Ok(Response::new())
}
