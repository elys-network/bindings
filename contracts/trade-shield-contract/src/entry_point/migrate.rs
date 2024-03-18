use std::collections::HashMap;

use cosmwasm_std::{Order, StdError};
use elys_bindings::trade_shield::{
    msg::MigrateMsg,
    states::{
        ACCOUNT_HISTORY_ADDRESS, PERPETUAL_ORDER, SPOT_ORDER, USER_PERPETUAL_ORDER, USER_SPOT_ORDER,
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

    let spot_orders = SPOT_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok().map(|r| r.1));

    let perpetual_orders = PERPETUAL_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok().map(|r| r.1));

    let mut user_spot_orders: HashMap<String, Vec<u64>> = HashMap::new();
    let mut user_perpetual_orders: HashMap<String, Vec<u64>> = HashMap::new();

    for order in spot_orders {
        let owner = order.owner_address.to_string();
        let mut ids = match user_spot_orders.get(&owner) {
            Some(ids) => ids.to_owned(),
            None => vec![],
        };
        ids.push(order.order_id);
        user_spot_orders.insert(owner, ids);
    }

    for order in perpetual_orders {
        let owner = order.owner.clone();
        let mut ids = match user_perpetual_orders.get(&owner) {
            Some(ids) => ids.to_owned(),
            None => vec![],
        };
        ids.push(order.order_id);
        user_perpetual_orders.insert(owner, ids);
    }

    user_spot_orders
        .into_iter()
        .map(|(addr, v)| USER_SPOT_ORDER.save(deps.storage, &addr, &v))
        .collect::<Result<Vec<()>, StdError>>()?;

    user_perpetual_orders
        .into_iter()
        .map(|(addr, v)| USER_PERPETUAL_ORDER.save(deps.storage, &addr, &v))
        .collect::<Result<Vec<()>, StdError>>()?;

    Ok(Response::new())
}
