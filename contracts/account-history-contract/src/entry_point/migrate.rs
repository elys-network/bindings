use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdResult};
use elys_bindings::account_history::msg::MigrationMsg;
use elys_bindings::types::PageRequest;
use elys_bindings::{ElysMsg, ElysQuery};

use crate::states::{PAGINATION, TRADE_SHIELD_ADDRESS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    msg: MigrationMsg,
) -> StdResult<Response<ElysMsg>> {
    if msg.trade_shield_address.is_some() {
        TRADE_SHIELD_ADDRESS.save(deps.storage, &msg.trade_shield_address)?;
    }

    let limit = match msg.limit {
        Some(limit) => limit,
        None => 10,
    };

    PAGINATION.save(
        deps.storage,
        &PageRequest {
            key: None,
            limit,
            reverse: false,
            offset: None,
            count_total: false,
        },
    )?;

    Ok(Response::new())
}
