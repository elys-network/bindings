use super::*;
use crate::states::*;
use cosmwasm_std::StdError;
use elys_bindings::ElysQuery;
use msg::InstantiateMsg;

use cw2::set_contract_version;

pub const CONTRACT_NAME: &str = "financial-snapshot";

pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let ver = std::env::var("VERSION");
    if ver.is_err() {
        return Err(StdError::generic_err("version read error"));
    }
    let contract_version: String = ver.unwrap();
    set_contract_version(deps.storage, CONTRACT_NAME, contract_version)?;
    LIQUIDITY_POSITIONS.save(deps.storage, &vec![])?;

    Ok(Response::new())
}
