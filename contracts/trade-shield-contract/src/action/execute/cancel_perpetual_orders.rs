use std::collections::HashMap;

use cosmwasm_std::{to_json_binary, Coin, StdError};

use crate::helper::remove_perpetual_order;

use super::*;

pub fn cancel_perpetual_orders(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_ids: Option<Vec<u64>>,
    order_type: Option<PerpetualOrderType>,
) -> Result<Response<ElysMsg>, ContractError> {
    if PERPETUAL_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("perpetual endpoint are disable").into());
    }
    let orders: Vec<PerpetualOrder> = if let Some(ids) = &order_ids {
        if ids.is_empty() {
            return Err(StdError::generic_err("order_ids is defined empty").into());
        };
        let orders = ids
            .iter()
            .map(|id| PERPETUAL_ORDER.load(deps.storage, *id))
            .collect::<Result<Vec<PerpetualOrder>, StdError>>()?;

        if orders
            .iter()
            .any(|order| order.owner != info.sender.as_str())
        {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        if let Some(order) = orders.iter().find(|order| {
            order.status != Status::Pending
                || order.order_type == PerpetualOrderType::MarketOpen
                || order.order_type == PerpetualOrderType::MarketClose
        }) {
            return Err(ContractError::CancelStatusError {
                order_id: order.order_id,
                status: order.status.clone(),
            });
        }

        orders
    } else {
        let orders: Vec<PerpetualOrder> =
            match USER_PERPETUAL_ORDER.may_load(deps.storage, info.sender.as_str())? {
                Some(v) => v
                    .iter()
                    .filter_map(|id| match PERPETUAL_ORDER.load(deps.storage, *id) {
                        Ok(order)
                            if order.status == Status::Pending
                                && order.order_type != PerpetualOrderType::MarketOpen
                                && order.order_type != PerpetualOrderType::MarketClose =>
                        {
                            Some(order)
                        }
                        _ => None,
                    })
                    .collect(),
                None => vec![],
            };

        if orders.is_empty() {
            return Err(ContractError::StdError(StdError::not_found(
                "no order found for this user",
            )));
        };

        orders
    };

    let mut orders = filter_order_by_type(orders, order_type)?;

    for order in orders.iter_mut() {
        remove_perpetual_order(order.order_id, Status::Canceled, deps.storage)?;
    }

    let order_ids: Vec<u64> = orders.iter().map(|order| order.order_id).collect();

    let number_of_pending_order =
        NUMBER_OF_PENDING_ORDER.load(deps.storage)? - order_ids.len() as u64;
    NUMBER_OF_PENDING_ORDER.save(deps.storage, &number_of_pending_order)?;

    let refund_msg = make_refund_msg(orders, info.sender.to_string());

    Ok(Response::new()
        .add_message(refund_msg)
        .set_data(to_json_binary(&order_ids)?))
}

fn filter_order_by_type(
    orders: Vec<PerpetualOrder>,
    order_type: Option<PerpetualOrderType>,
) -> Result<Vec<PerpetualOrder>, ContractError> {
    let order_type = match order_type {
        Some(order_type) => order_type,
        None => return Ok(orders),
    };

    let filtered_order: Vec<PerpetualOrder> = orders
        .iter()
        .filter(|order| order.order_type == order_type)
        .cloned()
        .collect();

    if filtered_order.is_empty() {
        Err(ContractError::StdError(cosmwasm_std::StdError::not_found(
            "no order his this type",
        )))
    } else {
        Ok(filtered_order)
    }
}

fn make_refund_msg(orders: Vec<PerpetualOrder>, user: String) -> BankMsg {
    let orders_amount: Vec<Coin> = orders
        .into_iter()
        .filter_map(|order| {
            if order.order_type == PerpetualOrderType::LimitOpen {
                Some(order.collateral)
            } else {
                None
            }
        })
        .collect();

    let mut merged_amounts: HashMap<String, Coin> = HashMap::new();

    for order_amount in orders_amount {
        if let Some(entry) = merged_amounts.get_mut(&order_amount.denom) {
            entry.amount += order_amount.amount;
        } else {
            merged_amounts.insert(order_amount.denom.clone(), order_amount);
        }
    }

    let merged_amounts: Vec<Coin> = merged_amounts.values().cloned().collect();

    BankMsg::Send {
        to_address: user,
        amount: merged_amounts,
    }
}
