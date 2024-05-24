use self::instantiate::CONTRACT_NAME;

use super::*;
use cosmwasm_std::StdError;
use cw2::set_contract_version;
use elys_bindings::trade_shield::{
    msg::MigrateMsg,
    states::{
        ACCOUNT_HISTORY_ADDRESS, LEVERAGE_ENABLED, LIMIT_PROCESS_ORDER, MARKET_ORDER_ENABLED,
        PARAMS_ADMIN, PERPETUAL_ENABLED, PROCESS_ORDERS_ENABLED, REWARD_ENABLED, STAKE_ENABLED,
        SWAP_ENABLED,
    },
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    msg: MigrateMsg,
) -> StdResult<Response<ElysMsg>> {
    if msg.account_history_address.is_some() {
        ACCOUNT_HISTORY_ADDRESS.save(deps.storage, &msg.account_history_address)?;
    }

    let admin = "elys16xffmfa6k45j340cx5zyp66lqvuw62a0neaa7w".to_string();
    PARAMS_ADMIN.save(deps.storage, &admin)?;

    let state = true;
    let limit_process_order: Option<u128> = Some(100);
    STAKE_ENABLED.save(deps.storage, &state)?;
    MARKET_ORDER_ENABLED.save(deps.storage, &state)?;
    SWAP_ENABLED.save(deps.storage, &state)?;
    PROCESS_ORDERS_ENABLED.save(deps.storage, &state)?;
    PERPETUAL_ENABLED.save(deps.storage, &state)?;
    REWARD_ENABLED.save(deps.storage, &state)?;
    LEVERAGE_ENABLED.save(deps.storage, &state)?;
    LIMIT_PROCESS_ORDER.save(deps.storage, &limit_process_order)?;

    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    let ver_env = std::env::var("VERSION");
    if ver_env.is_err() {
        return Err(StdError::generic_err("version read error"));
    }
    let contract_version: String = ver_env.unwrap();
    if ver.version.as_str() >= contract_version.as_str() {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    // set the new version
    set_contract_version(deps.storage, CONTRACT_NAME, contract_version)?;
    Ok(Response::new())
}
