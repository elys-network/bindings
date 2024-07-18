use self::instantiate::{CONTRACT_NAME, CONTRACT_VERSION};

use super::*;
use cosmwasm_std::StdError;
use cw2::set_contract_version;
use elys_bindings::trade_shield::{
    msg::MigrateMsg,
    states::{
        LEVERAGE_ENABLED, LIMIT_PROCESS_ORDER, MARKET_ORDER_ENABLED, PARAMS_ADMIN,
        PERPETUAL_ENABLED, PROCESS_ORDERS_ENABLED, REWARD_ENABLED, STAKE_ENABLED, SWAP_ENABLED,
    },
};
use semver::Version;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    msg: MigrateMsg,
) -> StdResult<Response<ElysMsg>> {
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

    match std::env::var("IS_TEST_ENV") {
        Ok(val) => {
            if val == "TESTING" {
                return Ok(Response::new());
            }
        }
        Err(_e) => (),
    }

    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }

    let new_contract_version = Version::parse(CONTRACT_VERSION).unwrap();
    let actual_contract_version = Version::parse(ver.version.as_str()).unwrap();

    if new_contract_version.le(&actual_contract_version) {
        let err_version: String = format!(
            "Error the version of trade-shield-contract {} has to be upper to {}",
            new_contract_version.to_string(),
            actual_contract_version.to_string()
        );
        return Err(StdError::generic_err(err_version).into());
    }
    // set the new version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}
