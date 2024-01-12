use cosmwasm_std::Empty;

use crate::states::TRADE_SHIELD_ADDRESS;

use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<ElysQuery>, _env: Env, _msg: Empty) -> StdResult<Response<ElysMsg>> {
    TRADE_SHIELD_ADDRESS.save(
        deps.storage,
        &"elys1m3hduhk4uzxn8mxuvpz02ysndxfwgy5mq60h4c34qqn67xud584qeee3m4".to_string(),
    )?;
    Ok(Response::new())
}
