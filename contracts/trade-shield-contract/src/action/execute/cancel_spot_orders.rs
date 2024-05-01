use std::collections::HashMap;

use cosmwasm_std::{to_json_binary, Coin, StdError};

use super::*;
use crate::helper::remove_spot_order;

pub fn cancel_spot_orders(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_ids: Option<Vec<u64>>,
    order_type: Option<SpotOrderType>,
) -> Result<Response<ElysMsg>, ContractError> {
    if SWAP_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("swap is disable").into());
    }
    let orders: Vec<SpotOrder> = if let Some(ids) = &order_ids {
        if ids.is_empty() {
            return Err(StdError::generic_err("order_ids is defined empty").into());
        };
        let orders = ids
            .iter()
            .map(|id| SPOT_ORDER.load(deps.storage, *id))
            .collect::<Result<Vec<SpotOrder>, StdError>>()?;

        if orders
            .iter()
            .any(|order| order.owner_address != info.sender.as_str())
        {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        if let Some(order) = orders.iter().find(|order| {
            order.status != Status::Pending || order.order_type == SpotOrderType::MarketBuy
        }) {
            return Err(ContractError::CancelStatusError {
                order_id: order.order_id,
                status: order.status.clone(),
            });
        }

        orders
    } else {
        let orders: Vec<SpotOrder> =
            match USER_SPOT_ORDER.may_load(deps.storage, info.sender.as_str())? {
                Some(v) => v
                    .iter()
                    .filter_map(|id| match SPOT_ORDER.load(deps.storage, *id) {
                        Ok(order)
                            if order.status == Status::Pending
                                && order.order_type != SpotOrderType::MarketBuy =>
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
        remove_spot_order(order.order_id, Status::Canceled, deps.storage)?;
    }

    let order_ids: Vec<u64> = orders.iter().map(|order| order.order_id).collect();

    let refund_msg = make_refund_msg(orders, info.sender.to_string());

    Ok(Response::new()
        .add_message(refund_msg)
        .set_data(to_json_binary(&order_ids)?))
}

fn filter_order_by_type(
    orders: Vec<SpotOrder>,
    order_type: Option<SpotOrderType>,
) -> Result<Vec<SpotOrder>, ContractError> {
    let order_type = match order_type {
        Some(order_type) => order_type,
        None => return Ok(orders),
    };

    let filtered_order: Vec<SpotOrder> = orders
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
