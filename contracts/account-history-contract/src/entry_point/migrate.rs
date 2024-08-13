use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdResult, Timestamp};
use cw_utils::Expiration;
use elys_bindings::account_history::msg::MigrationMsg;
// use elys_bindings::account_history::types::Metadata;
use elys_bindings::{ElysMsg, ElysQuery};

use crate::states::{DELETE_EPOCH, DELETE_OLD_DATA_ENABLED, PROCESSED_ACCOUNT_PER_BLOCK};
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    msg: MigrationMsg,
) -> StdResult<Response<ElysMsg>> {
    // PROCESSED_ACCOUNT_PER_BLOCK
    let limit = match msg.limit {
        Some(limit) => limit,
        None => 1,
    };

    PROCESSED_ACCOUNT_PER_BLOCK.save(deps.storage, &limit)?;
    DELETE_OLD_DATA_ENABLED.save(deps.storage, &true)?;
    DELETE_EPOCH.save(deps.storage, &1000u64)?;

    // METADATA
    // let querier = ElysQuerier::new(&deps.querier);

    // let metadata = Metadata::collect(&querier)?;

    // METADATA.save(deps.storage, &metadata)?;

    // RESPONSE

    Ok(Response::new())
}
