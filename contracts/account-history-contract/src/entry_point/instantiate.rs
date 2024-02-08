use elys_bindings::types::PageRequest;

use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult};
use elys_bindings::{ElysMsg, ElysQuery};

use crate::msg::InstantiateMsg;
use crate::states::{EXPIRATION, INTERVAL, PAGINATION, TRADE_SHIELD_ADDRESS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<ElysMsg>> {
    INTERVAL.save(deps.storage, &msg.interval)?;
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
    TRADE_SHIELD_ADDRESS.save(deps.storage, &msg.trade_shield_address)?;
    Ok(Response::new())
}
