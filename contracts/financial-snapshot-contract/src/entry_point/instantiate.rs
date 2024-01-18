use super::*;
use crate::{bindings::query::ElysQuery, states::*, types::*};
use msg::InstantiateMsg;

pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    REWARDS.save(deps.storage, "", &Reward::init())?;
    LIQUIDITY_POSITIONS.save(deps.storage, &vec![])?;

    Ok(Response::new())
}
