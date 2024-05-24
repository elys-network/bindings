use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdResult, Timestamp};
use cw2::set_contract_version;
use cw_utils::Expiration;
use elys_bindings::account_history::msg::MigrationMsg;
// use elys_bindings::account_history::types::Metadata;
use elys_bindings::{ElysMsg, /*ElysQuerier,*/ ElysQuery};

use crate::states::{EXPIRATION, HISTORY, PROCESSED_ACCOUNT_PER_BLOCK, TRADE_SHIELD_ADDRESS};

use super::instantiate::{CONTRACT_NAME, CONTRACT_VERSION};

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

    // PROCESSED_ACCOUNT_PER_BLOCK
    let limit = match msg.limit {
        Some(limit) => limit,
        None => 1,
    };

    PROCESSED_ACCOUNT_PER_BLOCK.save(deps.storage, &limit)?;

    //clear Hitstory since we removed a field from the Snapshot to prevent panic
    HISTORY.clear(deps.storage);

    // METADATA
    // let querier = ElysQuerier::new(&deps.querier);

    // let metadata = Metadata::collect(&querier)?;

    // METADATA.save(deps.storage, &metadata)?;

    // RESPONSE

    // Uncomment after first migration
    // let ver = cw2::get_contract_version(deps.storage)?;
    // // ensure we are migrating from an allowed contract
    // if ver.contract != CONTRACT_NAME {
    //     return Err(StdError::generic_err("Can only upgrade from same type").into());
    // }
    // if ver.version.as_str() >= CONTRACT_VERSION {
    //     return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    // }

    // set the new version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}
