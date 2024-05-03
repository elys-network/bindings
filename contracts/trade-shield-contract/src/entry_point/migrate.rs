use super::*;
use cosmwasm_std::Storage;
use elys_bindings::trade_shield::{
    msg::MigrateMsg,
    states::{
        ACCOUNT_HISTORY_ADDRESS, LEVERAGE_ENABLED, LIMIT_PROCESS_ORDER, MARKET_ORDER_ENABLED,
        PARAMS_ADMIN, PENDING_SPOT_ORDER, PERPETUAL_ENABLED, PROCESS_ORDERS_ENABLED,
        REWARD_ENABLED, SORTED_PENDING_PERPETUAL_ORDER, SPOT_ORDER, STAKE_ENABLED, SWAP_ENABLED,
    },
    types::{SpotOrder, Status},
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

    let spot_orders: Vec<SpotOrder> = SPOT_ORDER
        .prefix_range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|result| {
            if let Ok((_, order)) = result {
                if matches!(order.status, Status::Pending) {
                    return Some(order);
                }
            }
            None
        })
        .collect();

    for mut order in spot_orders {
        let pending_order = PENDING_SPOT_ORDER.may_load(deps.storage, order.order_id)?;
        if pending_order.is_none() {
            cancel_order(&mut order, deps.storage)?;
            continue;
        }

        let key = order.gen_key()?;
        if let Some(ids) = SORTED_PENDING_PERPETUAL_ORDER.may_load(deps.storage, key.as_str())? {
            if let Some(&id) = ids.iter().find(|&&id| id == order.order_id) {
                if pending_order.is_none() {
                    let filtered_ids: Vec<u64> =
                        ids.iter().filter(|&&id_| id_ != id).cloned().collect();
                    SORTED_PENDING_PERPETUAL_ORDER.save(
                        deps.storage,
                        key.as_str(),
                        &filtered_ids,
                    )?;
                }
            } else {
                cancel_order(&mut order, deps.storage)?;
            }
        } else {
            cancel_order(&mut order, deps.storage)?;
        }
    }

    Ok(Response::new())
}

fn cancel_order(order: &mut SpotOrder, storage: &mut dyn Storage) -> StdResult<()> {
    order.status = Status::Canceled;
    SPOT_ORDER.save(storage, order.order_id, order)?;
    PENDING_SPOT_ORDER.remove(storage, order.order_id);
    Ok(())
}
