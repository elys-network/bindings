use cosmwasm_std::Order;
use elys_bindings::trade_shield::{
    msg::MigrateMsg,
    states::{
        ACCOUNT_HISTORY_ADDRESS, LEVERAGE_ENABLED, LIMIT_PROCESS_ORDER, MARKET_ORDER_ENABLED,
        PARAMS_ADMIN, PENDING_SPOT_ORDER, PERPETUAL_ENABLED, PROCESS_ORDERS_ENABLED,
        REWARD_ENABLED, SORTED_PENDING_SPOT_ORDER, STAKE_ENABLED, SWAP_ENABLED,
    },
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

    let admin = "elys16xffmfa6k45j340cx5zyp66lqvuw62a0neaa7w".to_string();
    PARAMS_ADMIN.save(deps.storage, &admin)?;

    let state = true;
    let limit_process_order: Option<u128> = Some(100);
    STAKE_ENABLED.save(deps.storage, &state)?;
    MARKET_ORDER_ENABLED.save(deps.storage, &state)?;
    SWAP_ENABLED.save(deps.storage, &state)?;
    PROCESS_ORDERS_ENABLED.save(deps.storage, &state)?;
    PERPETUAL_ENABLED.save(deps.storage, &state)?;
    REWARD_ENABLED.save(deps.storage, &state)?;
    LEVERAGE_ENABLED.save(deps.storage, &state)?;
    LIMIT_PROCESS_ORDER.save(deps.storage, &limit_process_order)?;

    let spot_orders_ids: Vec<(String, Vec<u64>)> = SORTED_PENDING_SPOT_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok())
        .collect();

    for (key, list) in spot_orders_ids {
        let mut existing: Vec<u64> = vec![];
        for id in list {
            if PENDING_SPOT_ORDER.may_load(deps.storage, id)?.is_some() {
                existing.push(id);
            }
        }
        SORTED_PENDING_SPOT_ORDER.save(deps.storage, key.as_str(), &existing)?;
    }

    Ok(Response::new())
}
