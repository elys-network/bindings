use cosmwasm_std::entry_point;
use cosmwasm_std::Empty;
use cosmwasm_std::StdError;
use cw2::set_contract_version;

use self::instantiate::CONTRACT_NAME;

use super::*;
use elys_bindings::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<ElysQuery>, _env: Env, _msg: Empty) -> StdResult<Response<ElysMsg>> {
    let ver_env = std::env::var("VERSION");
    if ver_env.is_err() {
        return Err(StdError::generic_err("version read error"));
    }
    let contract_version: String = ver_env.unwrap();
    // uncomment after first migration
    // let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    // if ver.version.as_str() >= contract_version.as_str() {
    //     return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    // }

    // set the new version
    set_contract_version(deps.storage, CONTRACT_NAME, contract_version)?;
    Ok(Response::new())
}
