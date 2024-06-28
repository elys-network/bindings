use super::*;
use crate::states::*;
use msg::InstantiateMsg;

use cw2::set_contract_version;

pub const CONTRACT_NAME: &str = "trade-shield";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<ElysMsg>> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    MAX_REPLY_ID.save(deps.storage, &0)?;
    SPOT_ORDER_MAX_ID.save(deps.storage, &0)?;
    PERPETUAL_ORDER_MAX_ID.save(deps.storage, &0)?;
    ACCOUNT_HISTORY_ADDRESS.save(deps.storage, &msg.account_history_address)?;

    let admin = "elys16xffmfa6k45j340cx5zyp66lqvuw62a0neaa7w".to_string();
    PARAMS_ADMIN.save(deps.storage, &admin)?;

    let state = true;
    MARKET_ORDER_ENABLED.save(deps.storage, &state)?;
    STAKE_ENABLED.save(deps.storage, &state)?;
    SWAP_ENABLED.save(deps.storage, &state)?;
    PROCESS_ORDERS_ENABLED.save(deps.storage, &state)?;
    PERPETUAL_ENABLED.save(deps.storage, &state)?;
    REWARD_ENABLED.save(deps.storage, &state)?;
    LEVERAGE_ENABLED.save(deps.storage, &state)?;
    LIMIT_PROCESS_ORDER.save(deps.storage, &None)?;
    NUMBER_OF_PENDING_ORDER.save(deps.storage, &0)?;
    NUMBER_OF_EXECUTED_ORDER.save(deps.storage, &0)?;
    Ok(Response::new())
}
