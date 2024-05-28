use self::instantiate::{CONTRACT_NAME, CONTRACT_VERSION};

use super::*;
use cw2::set_contract_version;
use elys_bindings::trade_shield::{
    msg::MigrateMsg,
    states::{ACCOUNT_HISTORY_ADDRESS, PARAMS_ADMIN},
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
