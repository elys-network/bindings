use cosmwasm_std::Empty;
use cosmwasm_std::{entry_point, StdError};
use cw2::set_contract_version;

use self::instantiate::{CONTRACT_NAME, CONTRACT_VERSION};

use super::*;
use elys_bindings::*;
use semver::Version;
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<ElysQuery>, _env: Env, _msg: Empty) -> StdResult<Response<ElysMsg>> {
    // Uncomment after first migration
    // let ver = cw2::get_contract_version(deps.storage)?;
    // // ensure we are migrating from an allowed contract
    // if ver.contract != CONTRACT_NAME {
    //     return Err(StdError::generic_err("Can only upgrade from same type").into());
    // }
    // if ver.version.as_str() >= CONTRACT_VERSION {
    //     return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    // }

    let ver = cw2::get_contract_version(deps.storage)?;
    let new_contract_version = Version::parse(CONTRACT_VERSION).unwrap();
    let actual_contract_version = Version::parse(ver.version.as_str()).unwrap();

    if new_contract_version.le(&actual_contract_version) {
        let err_version: String = format!(
            "Error the version of financial-snapshot-contract {} has to be upper to {}",
            new_contract_version.to_string(),
            actual_contract_version.to_string()
        );

        return Err(StdError::generic_err(err_version).into());
    }

    // set the new version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}
