use elys_bindings::trade_shield::{
    msg::MigrateMsg,
    states::{ACCOUNT_HISTORY_ADDRESS, MARKET_ORDER, STAKE_ENDPOINT},
};

use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    msg: MigrateMsg,
) -> StdResult<Response<ElysMsg>> {
    if msg.account_history_address.is_some() {
        ACCOUNT_HISTORY_ADDRESS.save(deps.storage, &msg.account_history_address)?;
    }

    let state = false;
    MARKET_ORDER.save(deps.storage, &state)?;
    STAKE_ENDPOINT.save(deps.storage, &state)?;

    Ok(Response::new())
}
