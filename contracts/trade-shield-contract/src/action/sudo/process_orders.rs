use crate::{
    helper::{get_discount, remove_perpetual_order},
    msg::ReplyType,
};
use cosmwasm_std::{
    coin, to_json_binary, Decimal, Int128, OverflowError, QuerierWrapper, StdError, StdResult,
    Storage, SubMsg,
};
use elys_bindings::query_resp::{Entry, QueryGetEntryResponse};

use super::*;

pub fn process_orders(
    deps: DepsMut<ElysQuery>,
    env: Env,
) -> Result<Response<ElysMsg>, ContractError> {
    let spot_orders: Vec<(String, Vec<u64>)> = if SWAP_ENABLED.load(deps.storage)? {
        SORTED_PENDING_SPOT_ORDER
            .prefix_range(deps.storage, None, None, Order::Ascending)
            .filter_map(|res| res.ok())
            .collect()
    } else {
        vec![]
    };

    let mut n_spot_order = LIMIT_PROCESS_ORDER.load(deps.storage)?;
    let mut n_perpetual_order = n_spot_order.clone();

    let perpetual_orders: Vec<(String, Vec<u64>)> = if PERPETUAL_ENABLED.load(deps.storage)? {
        SORTED_PENDING_PERPETUAL_ORDER
            .prefix_range(deps.storage, None, None, Order::Ascending)
            .filter_map(|res| res.ok())
            .collect()
    } else {
        vec![]
    };

    let mut reply_info_id = MAX_REPLY_ID.load(deps.storage)?;

    let querier = ElysQuerier::new(&deps.querier);
    let mut submsgs: Vec<SubMsg<ElysMsg>> = vec![];
    let mut bank_msgs: Vec<BankMsg> = vec![];

    let QueryGetEntryResponse {
        entry: Entry {
            denom: _usdc_denom, ..
        },
    } = querier.get_asset_profile("uusdc".to_string())?;

    for (key, order_ids) in spot_orders.iter() {
        if n_spot_order == Some(0) {
            break;
        }

        let (order_type, base_denom, quote_denom) = SpotOrder::from_key(key.as_str())?;

        if order_type == SpotOrderType::MarketBuy {
            SORTED_PENDING_SPOT_ORDER.remove(deps.storage, key.as_str());
            continue;
        }

        let market_price =
            match querier.get_asset_price_from_denom_in_to_denom_out(&base_denom, &quote_denom) {
                Ok(market_price) => {
                    if order_type == SpotOrderType::LimitBuy {
                        match Decimal::one().checked_div(market_price.clone()) {
                            Ok(market_price) => market_price,
                            Err(_) => {
                                continue;
                            }
                        }
                    } else {
                        market_price
                    }
                }
                Err(_) => {
                    bank_msgs.extend(cancel_spot_orders(deps.storage, key, order_ids, None)?);
                    continue;
                }
            };
        let closest_spot_price = SpotOrder::binary_search(&market_price, deps.storage, &order_ids)?;
        let routes = match querier.amm_swap_estimation_by_denom(
            &coin(1000000, &base_denom),
            &base_denom,
            &quote_denom,
            &Decimal::zero(),
        ) {
            Ok(r) => match r.in_route {
                Some(routes) => routes,
                None => {
                    bank_msgs.extend(cancel_spot_orders(deps.storage, key, order_ids, None)?);
                    continue;
                }
            },
            Err(_) => {
                bank_msgs.extend(cancel_spot_orders(deps.storage, key, order_ids, None)?);
                continue;
            }
        };

        let orders_to_process: Vec<u64> = split_spot_order(
            closest_spot_price,
            &mut n_spot_order,
            order_type,
            market_price,
            order_ids.to_owned(),
            deps.storage,
        )?;

        process_spot_order(
            routes,
            orders_to_process,
            &mut submsgs,
            env.contract.address.as_str(),
            &mut reply_info_id,
            deps.storage,
            deps.querier,
        )?;
    }

    for (key, order_ids) in perpetual_orders.iter() {
        if n_perpetual_order == Some(0) {
            break;
        }
        let (order_position_type, order_type, base_denom, quote_denom) =
            PerpetualOrderV2::from_key(key.as_str())?;

        //get the price in usdc
        let market_price =
            match querier.get_asset_price_from_denom_in_to_denom_out(&quote_denom, &base_denom) {
                Ok(market_price) => market_price,
                Err(_) => {
                    cancel_perpetual_orders(deps.storage, key, &order_ids, None)?;
                    continue;
                }
            };

        let closest_index = PerpetualOrderV2::binary_search(
            &Some(OrderPrice {
                base_denom,
                quote_denom,
                rate: market_price.clone(),
            }),
            deps.storage,
            &order_ids,
        )?;

        let order_to_execute = split_perpetual_order(
            closest_index,
            &mut n_perpetual_order,
            &order_position_type,
            order_type,
            market_price,
            order_ids.to_owned(),
            deps.storage,
        )?;

        process_perpetual_order(
            order_to_execute,
            &mut submsgs,
            &mut reply_info_id,
            deps.storage,
            &querier,
            env.contract.address.as_str(),
        )?;
    }

    MAX_REPLY_ID.save(deps.storage, &reply_info_id)?;

    let resp = if bank_msgs.is_empty() {
        Response::new().add_submessages(submsgs)
    } else {
        Response::new()
            .add_submessages(submsgs)
            .add_messages(bank_msgs)
    };

    Ok(resp)
}

fn process_perpetual_order(
    orders_ids: Vec<u64>,
    submsgs: &mut Vec<SubMsg<ElysMsg>>,
    reply_info_id: &mut u64,
    storage: &mut dyn Storage,
    querier: &ElysQuerier<'_>,
    creator: &str,
) -> StdResult<()> {
    for id in orders_ids {
        let order = PENDING_PERPETUAL_ORDER_V2.load(storage, id)?;

        let (msg, reply_type) = if order.order_type == PerpetualOrderType::LimitOpen {
            (
                ElysMsg::perpetual_open_position(
                    creator,
                    order.collateral.clone(),
                    &order.trading_asset,
                    order.position.clone(),
                    order.leverage.clone(),
                    order.take_profit_price.clone(),
                    &order.owner,
                ),
                ReplyType::PerpetualBrokerOpen,
            )
        } else {
            let mtp = match querier
                .mtp(order.owner.clone(), order.position_id.unwrap())?
                .mtp
            {
                Some(mtp) => mtp,
                None => {
                    remove_perpetual_order(id, Status::Canceled, storage, None)?;
                    continue;
                }
            };

            let amount = mtp.mtp.custody.i128();
            (
                ElysMsg::perpetual_close_position(
                    creator,
                    order.position_id.unwrap(),
                    amount,
                    &order.owner,
                ),
                ReplyType::PerpetualBrokerClose,
            )
        };

        *reply_info_id = match reply_info_id.checked_add(1) {
            Some(id) => id,
            None => {
                return Err(StdError::overflow(OverflowError::new(
                    cosmwasm_std::OverflowOperation::Add,
                    "reply_info_max_id",
                    "increment one",
                ))
                .into())
            }
        };

        let reply_info = ReplyInfo {
            id: *reply_info_id,
            reply_type,
            data: Some(to_json_binary(&order.order_id)?),
        };
        submsgs.push(SubMsg::reply_always(msg, *reply_info_id));

        REPLY_INFO.save(storage, *reply_info_id, &reply_info)?;
    }

    Ok(())
}

fn split_perpetual_order(
    closest_index: usize,
    n_perpetual_order: &mut Option<u128>,
    order_position_type: &PerpetualPosition,
    order_type: PerpetualOrderType,
    market_price: Decimal,
    ids: Vec<u64>,
    storage: &mut dyn Storage,
) -> StdResult<Vec<u64>> {
    if ids.is_empty() {
        return Ok(vec![]);
    }

    let order_price = if closest_index < ids.len() {
        Some(
            PENDING_PERPETUAL_ORDER_V2
                .load(storage, ids[closest_index])?
                .trigger_price
                .unwrap()
                .rate,
        )
    } else {
        None
    };

    let ids_to_process = match (order_price, order_type, order_position_type) {
        (Some(price), PerpetualOrderType::LimitOpen, PerpetualPosition::Long) => {
            if market_price <= price {
                Ok(ids.get(closest_index..).unwrap_or(&[]).to_vec())
            } else {
                Ok(ids.get(closest_index + 1..).unwrap_or(&[]).to_vec())
            }
        }
        (None, PerpetualOrderType::LimitOpen, PerpetualPosition::Long) => Ok(vec![]),
        (Some(price), PerpetualOrderType::LimitOpen, PerpetualPosition::Short) => {
            if market_price >= price {
                Ok(ids.get(..=closest_index).unwrap_or(&[]).to_vec())
            } else {
                Ok(ids.get(..closest_index).unwrap_or(&[]).to_vec())
            }
        }
        (None, PerpetualOrderType::LimitOpen, PerpetualPosition::Short) => Ok(ids),
        (Some(price), PerpetualOrderType::LimitClose, PerpetualPosition::Long) => {
            if market_price >= price {
                Ok(ids.get(..=closest_index).unwrap_or(&[]).to_vec())
            } else {
                Ok(ids.get(..closest_index).unwrap_or(&[]).to_vec())
            }
        }
        (None, PerpetualOrderType::LimitClose, PerpetualPosition::Long) => Ok(ids),
        (Some(price), PerpetualOrderType::LimitClose, PerpetualPosition::Short) => {
            if market_price <= price {
                Ok(ids.get(closest_index..).unwrap_or(&[]).to_vec())
            } else {
                Ok(ids.get(closest_index + 1..).unwrap_or(&[]).to_vec())
            }
        }
        (None, PerpetualOrderType::LimitClose, PerpetualPosition::Short) => Ok(vec![]),
        (Some(price), PerpetualOrderType::StopLoss, PerpetualPosition::Long) => {
            if market_price <= price {
                Ok(ids.get(closest_index..).unwrap_or(&[]).to_vec())
            } else {
                Ok(ids.get(closest_index + 1..).unwrap_or(&[]).to_vec())
            }
        }
        (None, PerpetualOrderType::StopLoss, PerpetualPosition::Long) => Ok(vec![]),
        (Some(price), PerpetualOrderType::StopLoss, PerpetualPosition::Short) => {
            if market_price >= price {
                Ok(ids.get(..=closest_index).unwrap_or(&[]).to_vec())
            } else {
                Ok(ids.get(..closest_index).unwrap_or(&[]).to_vec())
            }
        }
        (None, PerpetualOrderType::StopLoss, PerpetualPosition::Short) => Ok(ids),
        _ => Err(StdError::generic_err("process market order")),
    }?;

    if let Some(n) = n_perpetual_order {
        if *n < ids_to_process.len() as u128 {
            let max_order = *n as usize;
            *n = 0;
            let new_ids_to_process = ids_to_process.get(0..max_order).unwrap_or(&[]).to_vec();
            return Ok(new_ids_to_process);
        } else {
            *n -= ids_to_process.len() as u128;
        }
    };

    Ok(ids_to_process)
    // (PerpetualOrderType::LimitOpen, PerpetualPosition::Long) => market_price <= order_price,
    // (PerpetualOrderType::LimitOpen, PerpetualPosition::Short) => market_price >= order_price,
    // (PerpetualOrderType::LimitClose, PerpetualPosition::Long) => market_price >= order_price,
    // (PerpetualOrderType::LimitClose, PerpetualPosition::Short) => market_price <= order_price,
    // (PerpetualOrderType::StopLoss, PerpetualPosition::Long) => market_price <= order_price,
    // (PerpetualOrderType::StopLoss, PerpetualPosition::Short) => market_price >= order_price,
}

fn split_spot_order(
    closest_index: usize,
    n_spot_order: &mut Option<u128>,
    order_type: SpotOrderType,
    market_price: Decimal,
    ids: Vec<u64>,
    storage: &mut dyn Storage,
) -> StdResult<Vec<u64>> {
    if ids.is_empty() {
        return Ok(vec![]);
    }

    let order_price = if closest_index < ids.len() {
        Some(
            PENDING_SPOT_ORDER
                .load(storage, ids[closest_index])?
                .order_price
                .rate,
        )
    } else {
        None
    };

    let id_to_process = match (order_type, order_price) {
        (SpotOrderType::StopLoss, Some(order_price)) => {
            if market_price <= order_price {
                Ok(ids.get(closest_index..).unwrap_or(&[]).to_vec())
            } else {
                Ok(ids.get(closest_index + 1..).unwrap_or(&[]).to_vec())
            }
        }
        (SpotOrderType::StopLoss, _) => Ok(vec![]),
        (SpotOrderType::LimitSell, Some(order_price)) => {
            if market_price >= order_price {
                Ok(ids.get(..=closest_index).unwrap_or(&[]).to_vec())
            } else {
                Ok(ids.get(..closest_index).unwrap_or(&[]).to_vec())
            }
        }
        (SpotOrderType::LimitSell, _) => Ok(ids),
        (SpotOrderType::LimitBuy, Some(order_price)) => {
            if market_price <= order_price {
                Ok(ids.get(closest_index..).unwrap_or(&[]).to_vec())
            } else {
                Ok(ids.get(closest_index + 1..).unwrap_or(&[]).to_vec())
            }
        }
        (SpotOrderType::LimitBuy, _) => Ok(vec![]),
        _ => Err(StdError::generic_err("Unsupported market order type")),
    }?;

    if let Some(n) = n_spot_order {
        if *n < id_to_process.len() as u128 {
            let max_order = *n as usize;
            *n = 0;
            let new_ids_to_process = id_to_process.get(0..max_order).unwrap_or(&[]).to_vec();
            return Ok(new_ids_to_process);
        } else {
            *n -= id_to_process.len() as u128;
        }
    };

    Ok(id_to_process)
    // SpotOrderType::StopLoss => market_price <= order_price,
    // SpotOrderType::LimitSell => market_price >= order_price,
    // SpotOrderType::LimitBuy => market_price <= order_price,
}

fn process_spot_order(
    routes: Vec<SwapAmountInRoute>,
    orders_ids: Vec<u64>,
    submsgs: &mut Vec<SubMsg<ElysMsg>>,
    sender: &str,
    reply_info_id: &mut u64,
    storage: &mut dyn Storage,
    querier: QuerierWrapper<'_, ElysQuery>,
) -> StdResult<()> {
    for id in orders_ids {
        let order = match PENDING_SPOT_ORDER.may_load(storage, id)? {
            Some(order) => order,
            None => continue,
        };

        *reply_info_id = match reply_info_id.checked_add(1) {
            Some(id) => id,
            None => {
                return Err(StdError::overflow(OverflowError::new(
                    cosmwasm_std::OverflowOperation::Add,
                    "reply_info_max_id",
                    "increment one",
                ))
                .into())
            }
        };
        let reply_info = ReplyInfo {
            id: *reply_info_id,
            reply_type: ReplyType::SpotOrder,
            data: Some(to_json_binary(&order.order_id)?),
        };
        REPLY_INFO.save(storage, *reply_info_id, &reply_info)?;

        let discount = get_discount(querier, order.owner_address.to_string())?;

        let msg = ElysMsg::amm_swap_exact_amount_in(
            sender,
            &order.order_amount,
            &routes,
            Int128::zero(),
            discount,
            order.owner_address.as_str(),
        );

        submsgs.push(SubMsg::reply_always(msg, *reply_info_id));
    }

    Ok(())
}

fn cancel_spot_orders(
    storage: &mut dyn Storage,
    key: &str,
    ids: &Vec<u64>,
    to_remove: Option<Vec<usize>>,
) -> StdResult<Vec<BankMsg>> {
    let mut bank_msg: Vec<BankMsg> = vec![];

    let order_to_remove: Vec<u64> = if let Some(indexs) = to_remove {
        let mut ids_clone = ids.clone();
        for i in indexs.iter().rev() {
            ids_clone.remove(*i);
        }
        SORTED_PENDING_SPOT_ORDER.save(storage, key, &ids_clone)?;
        indexs.iter().map(|index| ids[*index]).collect()
    } else {
        SORTED_PENDING_SPOT_ORDER.save(storage, key, &vec![])?;
        ids.clone()
    };
    for id in order_to_remove {
        let mut spot_order = SPOT_ORDER.load(storage, id)?;
        spot_order.status = Status::Canceled;
        PENDING_SPOT_ORDER.remove(storage, id);
        SPOT_ORDER.save(storage, id, &spot_order)?;
        bank_msg.push(BankMsg::Send {
            to_address: spot_order.owner_address.to_string(),
            amount: vec![spot_order.order_amount],
        })
    }

    Ok(bank_msg)
}

fn cancel_perpetual_orders(
    storage: &mut dyn Storage,
    key: &str,
    ids: &Vec<u64>,
    to_remove: Option<Vec<usize>>,
) -> StdResult<Vec<BankMsg>> {
    let mut bank_msg: Vec<BankMsg> = vec![];

    let order_to_remove: Vec<u64> = if let Some(indexs) = to_remove {
        let mut ids_clone = ids.clone();
        for i in indexs.iter().rev() {
            ids_clone.remove(*i);
        }
        SORTED_PENDING_PERPETUAL_ORDER.save(storage, key, &ids_clone)?;
        indexs.iter().map(|index| ids[*index]).collect()
    } else {
        SORTED_PENDING_PERPETUAL_ORDER.save(storage, key, &vec![])?;
        ids.clone()
    };
    for id in order_to_remove {
        let mut perpetual_order = PERPETUAL_ORDER_V2.load(storage, id)?;
        perpetual_order.status = Status::Canceled;
        PENDING_PERPETUAL_ORDER_V2.remove(storage, id);
        PERPETUAL_ORDER_V2.save(storage, id, &perpetual_order)?;
        if perpetual_order.order_type == PerpetualOrderType::LimitOpen {
            bank_msg.push(BankMsg::Send {
                to_address: perpetual_order.owner,
                amount: vec![perpetual_order.collateral],
            })
        }
    }

    Ok(bank_msg)
}

fn _calculate_token_out_min_amount(_order: &SpotOrder) -> Int128 {
    // FIXME:
    // insteade we want to use the amount field from swap-estimation-by-denom that
    // include slippage and reduce it by 1% that should be our token out min amount to return here
    //
    // let SpotOrder {
    //     order_amount,
    //     order_price,
    //     ..
    // } = order;

    // let amount = if order_amount.denom == order_price.base_denom {
    //     order_amount.amount * order_price.rate
    // } else {
    //     order_amount.amount * Decimal::one().div(order_price.rate)
    // };

    // Int128::new((amount.u128()) as i128)
    Int128::zero()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::testing::MockStorage;

    use super::*;

    #[test]
    fn test_split_function() {
        let ids = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut n_spot_order = Some(5);
        let closest_index = 12;
        let order_type = SpotOrderType::LimitSell;
        let market_price = Decimal::from_str("0.1").unwrap();
        let mut mock_storage = MockStorage::new();

        let result = split_spot_order(
            closest_index,
            &mut n_spot_order,
            order_type,
            market_price,
            ids,
            &mut mock_storage,
        )
        .unwrap();

        assert_eq!(result, vec![0, 1, 2, 3, 4]);
        assert_eq!(n_spot_order, Some(0));
    }
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;
//
//     use cosmwasm_std::{coin, Addr, Timestamp};
//
//     use super::*;
//
//     #[test]
//     fn test_check_spot_order_limit_buy() {
//         // Arrange
//         let spot_order = SpotOrder {
//             order_type: SpotOrderType::LimitBuy,
//             order_id: 1,
//             order_price: OrderPrice {
//                 base_denom: "uatom".to_string(),
//                 quote_denom: "usdc".to_string(),
//                 rate: Decimal::from_str("0.1").unwrap(),
//             },
//             order_amount: coin(1000000, "usdc"),
//
//             owner_address: Addr::unchecked("elysd"),
//             order_target_denom: "uatom".to_string(),
//             status: Status::Pending,
//             date: Date {
//                 height: 5,
//                 time: Timestamp::from_seconds(5),
//             },
//             // Initialize the rest of the SpotOrder fields here
//         };
//         let market_price = Decimal::from_str("9.29").unwrap();
//
//         // Act
//         let result = check_spot_order(&spot_order, market_price);
//
//         // Assert
//         assert_eq!(result, false); // Change as needed
//     }
//
//     #[test]
//     fn test_check_spot_order_limit_sell() {
//         // Arrange
//         let spot_order = SpotOrder {
//             order_type: SpotOrderType::LimitSell,
//             order_id: 1,
//             order_price: OrderPrice {
//                 base_denom: "uatom".to_string(),
//                 quote_denom: "usdc".to_string(),
//                 rate: Decimal::from_str("0.111111").unwrap(),
//             },
//             order_amount: coin(1000000, "uatom"),
//
//             owner_address: Addr::unchecked("elysd"),
//             order_target_denom: "usdc".to_string(),
//             status: Status::Pending,
//             date: Date {
//                 height: 5,
//                 time: Timestamp::from_seconds(5),
//             },
//             // Initialize the rest of the SpotOrder fields here
//         };
//         let market_price = Decimal::from_str("9.29").unwrap();
//         // Act
//         let result = check_spot_order(&spot_order, market_price);
//
//         // Assert
//         assert_eq!(result, true); // Change as needed
//     }
// }
