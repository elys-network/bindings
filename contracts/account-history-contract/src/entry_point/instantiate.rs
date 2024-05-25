use cw2::set_contract_version;
use cw_utils::Expiration;
use elys_bindings::account_history::types::Metadata;

use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp};
use elys_bindings::{ElysMsg, ElysQuerier, ElysQuery};

use crate::msg::InstantiateMsg;
use crate::states::{
    EXPIRATION, METADATA, PROCESSED_ACCOUNT_PER_BLOCK, TRADE_SHIELD_ADDRESS, UPDATE_ACCOUNT_ENABLED,
};

// Version info, for migration info
pub const CONTRACT_NAME: &str = "account-history";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<ElysMsg>> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // EXPIRATION
    match msg.expiration {
        Some(expiration) => EXPIRATION.save(deps.storage, &expiration)?,
        None => EXPIRATION.save(
            deps.storage,
            &Expiration::AtTime(Timestamp::from_seconds(3 * 24 * 60 * 60)),
        )?,
    };

    // PROCESSED_ACCOUNT_PER_BLOCK
    let limit = match msg.limit {
        Some(limit) => limit,
        None => 1,
    };

    PROCESSED_ACCOUNT_PER_BLOCK.save(deps.storage, &limit)?;

    // TRADESHIELD ADDRESS
    TRADE_SHIELD_ADDRESS.save(deps.storage, &msg.trade_shield_address)?;

    // METADATA
    let querier = ElysQuerier::new(&deps.querier);

    let metadata = Metadata::collect(&querier)?;

    METADATA.save(deps.storage, &metadata)?;

    UPDATE_ACCOUNT_ENABLED.save(deps.storage, &true)?;

    // RESPONSE
    Ok(Response::new())
}
