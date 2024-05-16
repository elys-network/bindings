use super::*;
use cosmwasm_std::Order;
use elys_bindings::trade_shield::{
    msg::MigrateMsg,
    states::{
        ACCOUNT_HISTORY_ADDRESS, LEVERAGE_ENABLED, LIMIT_PROCESS_ORDER, MARKET_ORDER_ENABLED,
        NUMBER_OF_EXECUTED_ORDER, NUMBER_OF_PENDING_ORDER, PARAMS_ADMIN, PENDING_PERPETUAL_ORDER,
        PENDING_SPOT_ORDER, PERPETUAL_ENABLED, PERPETUAL_ORDER, PROCESS_ORDERS_ENABLED,
        REWARD_ENABLED, SPOT_ORDER, STAKE_ENABLED, SWAP_ENABLED,
    },
    types::Status,
};

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

    let number_of_pending_order = PENDING_SPOT_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok())
        .count() as u64
        + PENDING_PERPETUAL_ORDER
            .prefix_range(deps.storage, None, None, Order::Ascending)
            .filter_map(|res| res.ok())
            .count() as u64;

    let number_of_executed_order = SPOT_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| {
            if let Some((_, order)) = res.ok() {
                if order.status == Status::Executed {
                    Some(order)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .count() as u64
        + PERPETUAL_ORDER
            .prefix_range(deps.storage, None, None, Order::Ascending)
            .filter_map(|res| {
                if let Some((_, order)) = res.ok() {
                    if order.status == Status::Executed {
                        Some(order)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .count() as u64;
    NUMBER_OF_PENDING_ORDER.save(deps.storage, &number_of_pending_order)?;
    NUMBER_OF_EXECUTED_ORDER.save(deps.storage, &number_of_executed_order)?;
    Ok(Response::new())
}
