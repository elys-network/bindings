use cw_utils::Expiration;
use elys_bindings::account_history::types::Metadata;
use elys_bindings::types::PageRequest;

use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp};
use elys_bindings::{ElysMsg, ElysQuerier, ElysQuery};

use crate::msg::InstantiateMsg;
use crate::states::{EXPIRATION, METADATA, PAGINATION, TRADE_SHIELD_ADDRESS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<ElysMsg>> {
    // EXPIRATION
    match msg.expiration {
        Some(expiration) => EXPIRATION.save(deps.storage, &expiration)?,
        None => EXPIRATION.save(
            deps.storage,
            &Expiration::AtTime(Timestamp::from_seconds(7 * 24 * 60 * 60)),
        )?,
    };

    // PAGINATION
    let limit = match msg.limit {
        Some(limit) => limit,
        None => 1,
    };

    PAGINATION.save(
        deps.storage,
        &PageRequest {
            key: None,
            limit,
            reverse: false,
            offset: None,
            count_total: false,
        },
    )?;

    // TRADESHIELD ADDRESS
    TRADE_SHIELD_ADDRESS.save(deps.storage, &msg.trade_shield_address)?;

    // METADATA
    let querier = ElysQuerier::new(&deps.querier);

    let metadata = Metadata::collect(&querier)?;

    METADATA.save(deps.storage, &metadata)?;

    // RESPONSE
    Ok(Response::new())
}
