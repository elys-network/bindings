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

    Ok(Response::new())
}
