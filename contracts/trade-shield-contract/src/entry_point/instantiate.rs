use super::*;
use crate::states::*;
use msg::InstantiateMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<ElysMsg>> {
    MAX_REPLY_ID.save(deps.storage, &0)?;
    SPOT_ORDER_MAX_ID.save(deps.storage, &0)?;
    ACCOUNT_HISTORY_ADDRESS.save(deps.storage, &msg.account_history_address)?;

    let admin = "elys16xffmfa6k45j340cx5zyp66lqvuw62a0neaa7w".to_string();
    PARAMS_ADMIN.save(deps.storage, &admin)?;

    let state = false;
    MARKET_ORDER_ENABLED.save(deps.storage, &state)?;
    STAKE_ENABLED.save(deps.storage, &state)?;
    SWAP_ENABLED.save(deps.storage, &state)?;
    PROCESS_ORDERS_ENABLED.save(deps.storage, &state)?;
    PERPETUAL_ENABLED.save(deps.storage, &state)?;
    REWARD_ENABLE.save(deps.storage, &state)?;
    LEVERAGE_ENABLE.save(deps.storage, &state)?;
    Ok(Response::new())
}
