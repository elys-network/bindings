use super::*;
use crate::{bindings::query::ElysQuery, states::*, types::*};
use msg::InstantiateMsg;

pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    LIQUID_ASSETS.save(deps.storage, &vec![])?;
    PORTFOLIO.save(deps.storage, "", &Portfolio::init())?;
    REWARDS.save(deps.storage, "", &Reward::init())?;
    TOTAL_BALANCE.save(deps.storage, "", &TotalBalance::init())?;
    LIQUIDITY_POSITIONS.save(deps.storage, &vec![])?;

    Ok(Response::new())
}
