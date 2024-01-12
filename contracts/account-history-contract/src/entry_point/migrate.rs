use cosmwasm_std::Empty;

use crate::states::{TRADE_SHIELD_ADDRESS, VALUE_DENOM};

use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<ElysQuery>, _env: Env, _msg: Empty) -> StdResult<Response<ElysMsg>> {
    TRADE_SHIELD_ADDRESS.save(
        deps.storage,
        &"elys1m3hduhk4uzxn8mxuvpz02ysndxfwgy5mq60h4c34qqn67xud584qeee3m4".to_string(),
    )?;
    VALUE_DENOM.save(
        deps.storage,
        &"ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
    )?;
    Ok(Response::new())
}
