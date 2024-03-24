use std::collections::HashMap;

use crate::{
    states::*,
    types::{PerpetualOrder, SpotOrder, Status},
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use elys_bindings::{ElysMsg, ElysQuery};

#[cw_serde]
pub struct InstantiateMockMsg {
    pub spot_orders: Vec<SpotOrder>,
    pub perpetual_orders: Vec<PerpetualOrder>,
    pub account_history_address: Option<String>,
}

pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMockMsg,
) -> StdResult<Response<ElysMsg>> {
    let mut user_spot_orders: HashMap<String, Vec<u64>> = HashMap::new();
    let mut user_perpetual_orders: HashMap<String, Vec<u64>> = HashMap::new();

    for order in msg.spot_orders.iter() {
        let owner = order.owner_address.to_string();

        SPOT_ORDER.save(deps.storage, order.order_id, order)?;
        if order.status == Status::Pending {
            PENDING_SPOT_ORDER.save(deps.storage, order.order_id, order)?;

            let key = order.gen_key()?;
            let mut vec = SORTED_PENDING_SPOT_ORDER
                .may_load(deps.storage, key.as_str())?
                .unwrap_or(vec![]);
            let index = SpotOrder::binary_search(&order.order_price.rate, deps.storage, &vec)?;
            if vec.len() <= index {
                vec.push(order.order_id)
            } else {
                vec.insert(index, order.order_id);
            }
            SORTED_PENDING_SPOT_ORDER.save(deps.storage, key.as_str(), &vec)?;
        }
        let mut ids = match user_spot_orders.get(&owner) {
            Some(ids) => ids.to_owned(),
            None => vec![],
        };
        ids.push(order.order_id);
        user_spot_orders.insert(owner, ids);
    }
    for order in msg.perpetual_orders.iter() {
        let owner = order.owner.clone();

        PERPETUAL_ORDER.save(deps.storage, order.order_id, order)?;
        if order.status == Status::Pending {
            PENDING_PERPETUAL_ORDER.save(deps.storage, order.order_id, order)?;

            PENDING_PERPETUAL_ORDER.save(deps.storage, order.order_id, &order)?;
            let key = order.gen_key()?;
            let mut vec = SORTED_PENDING_PERPETUAL_ORDER
                .may_load(deps.storage, key.as_str())?
                .unwrap_or(vec![]);
            let index = order.binary_search(deps.storage, &vec)?;
            if vec.len() <= index {
                vec.push(order.order_id)
            } else {
                vec.insert(index, order.order_id);
            }
            SORTED_PENDING_PERPETUAL_ORDER.save(deps.storage, key.as_str(), &vec)?;
        }
        let mut ids = match user_perpetual_orders.get(&owner) {
            Some(ids) => ids.to_owned(),
            None => vec![],
        };
        ids.push(order.order_id);
        user_perpetual_orders.insert(owner, ids);
    }
    MAX_REPLY_ID.save(deps.storage, &0)?;
    SPOT_ORDER_MAX_ID.save(deps.storage, &0)?;
    ACCOUNT_HISTORY_ADDRESS.save(deps.storage, &msg.account_history_address)?;
    user_spot_orders
        .into_iter()
        .map(|(addr, v)| USER_SPOT_ORDER.save(deps.storage, &addr, &v))
        .collect::<Result<Vec<()>, StdError>>()?;

    user_perpetual_orders
        .into_iter()
        .map(|(addr, v)| USER_PERPETUAL_ORDER.save(deps.storage, &addr, &v))
        .collect::<Result<Vec<()>, StdError>>()?;

    let admin = "elys16xffmfa6k45j340cx5zyp66lqvuw62a0neaa7w".to_string();
    PARAMS_ADMIN.save(deps.storage, &admin)?;

    let state = true;
    STAKE_ENABLED.save(deps.storage, &state)?;
    MARKET_ORDER_ENABLED.save(deps.storage, &state)?;
    SWAP_ENABLED.save(deps.storage, &state)?;
    PROCESS_ORDERS_ENABLED.save(deps.storage, &state)?;
    PERPETUAL_ENABLED.save(deps.storage, &state)?;
    REWARD_ENABLED.save(deps.storage, &state)?;
    LEVERAGE_ENABLED.save(deps.storage, &state)?;
    LIMIT_PROCESS_ORDER.save(deps.storage, &None)?;

    Ok(Response::new())
}
