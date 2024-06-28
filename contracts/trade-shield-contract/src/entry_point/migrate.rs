use self::instantiate::{CONTRACT_NAME, CONTRACT_VERSION};

use super::*;
use cosmwasm_std::{Order, StdError};
use cw2::set_contract_version;
use elys_bindings::trade_shield::{
    msg::MigrateMsg,
    states::{
        ACCOUNT_HISTORY_ADDRESS, LEVERAGE_ENABLED, LIMIT_PROCESS_ORDER, MARKET_ORDER_ENABLED,
        PARAMS_ADMIN, PERPETUAL_ENABLED, PROCESS_ORDERS_ENABLED, REWARD_ENABLED, STAKE_ENABLED,
        SWAP_ENABLED,
    },
};
use trade_shield::{
    states::{
        CLOSE_PERPETUAL_ORDER, PENDING_PERPETUAL_ORDER, PERPETUAL_ORDER, PERPETUAL_ORDER_MAX_ID,
    },
    types::{PerpetualOrder, PerpetualOrderType, Status},
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

    let perpetual_max_id = PERPETUAL_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok().map(|r| r.0))
        .max_by_key(|id| *id);
    PERPETUAL_ORDER_MAX_ID.save(deps.storage, &perpetual_max_id.unwrap_or(0))?;

    let pending_perpetual_orders: Vec<PerpetualOrder> = PENDING_PERPETUAL_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| match res {
            Ok((_, order)) => {
                if order.status == Status::Pending
                    && (order.order_type == PerpetualOrderType::LimitClose
                        || order.order_type == PerpetualOrderType::StopLoss)
                {
                    Some(order)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect();

    for perpetual_order in pending_perpetual_orders {
        let mut pending_close_position = CLOSE_PERPETUAL_ORDER
            .may_load(deps.storage, perpetual_order.position_id.unwrap())?
            .unwrap_or_default();
        pending_close_position.push(perpetual_order.order_id);
        CLOSE_PERPETUAL_ORDER.save(
            deps.storage,
            perpetual_order.position_id.unwrap(),
            &pending_close_position,
        )?;
    }

    match std::env::var("IS_TEST_ENV") {
        Ok(val) => {
            if val == "TESTING" {
                return Ok(Response::new());
            }
        }
        Err(_e) => (),
    }

    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    if ver.version.as_str() >= CONTRACT_VERSION {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    // set the new version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}
