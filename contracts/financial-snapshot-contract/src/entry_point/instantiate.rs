use super::*;
use crate::states::*;
use elys_bindings::ElysQuery;
use msg::InstantiateMsg;

use cw2::set_contract_version;

const CONTRACT_NAME: &str = "financial-snapshot";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    LIQUIDITY_POSITIONS.save(deps.storage, &vec![])?;

    Ok(Response::new())
}
