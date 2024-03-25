use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdResult, Timestamp};
use cw_utils::Expiration;
use elys_bindings::account_history::msg::MigrationMsg;
// use elys_bindings::account_history::types::Metadata;
use elys_bindings::types::PageRequest;
use elys_bindings::{ElysMsg, /*ElysQuerier,*/ ElysQuery};

use crate::states::{EXPIRATION, /*METADATA,*/ PAGINATION, TRADE_SHIELD_ADDRESS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    msg: MigrationMsg,
) -> StdResult<Response<ElysMsg>> {
    // TRADESHIELD ADDRESS
    if msg.trade_shield_address.is_some() {
        TRADE_SHIELD_ADDRESS.save(deps.storage, &msg.trade_shield_address)?;
    }

    // EXPIRATION
    EXPIRATION.save(
        deps.storage,
        &Expiration::AtTime(Timestamp::from_seconds(3 * 24 * 60 * 60)),
    )?;

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

    // METADATA
    // let querier = ElysQuerier::new(&deps.querier);

    // let metadata = Metadata::collect(&querier)?;

    // METADATA.save(deps.storage, &metadata)?;

    // RESPONSE
    Ok(Response::new())
}
