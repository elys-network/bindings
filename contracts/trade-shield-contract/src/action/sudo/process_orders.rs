use crate::msg::ReplyType;
use cosmwasm_std::{
    to_json_binary, Decimal, Int128, OverflowError, StdError, StdResult, Storage, SubMsg,
};
use elys_bindings::query_resp::AmmSwapEstimationByDenomResponse;
use std::ops::Div;

use super::*;

pub fn process_orders(
    deps: DepsMut<ElysQuery>,
    env: Env,
) -> Result<Response<ElysMsg>, ContractError> {
    let spot_orders: Vec<SpotOrder> = PENDING_SPOT_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok().map(|r| r.1))
        .collect();

    let margin_orders: Vec<MarginOrder> = PENDING_MARGIN_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok().map(|r| r.1))
        .collect();
    let mut reply_info_id = MAX_REPLY_ID.load(deps.storage)?;

    let querier = ElysQuerier::new(&deps.querier);
    let mut submsgs: Vec<SubMsg<ElysMsg>> = vec![];
    let mut bank_msgs: Vec<BankMsg> = vec![];

    for spot_order in spot_orders.iter() {
        let amm_swap_estimation = match querier.amm_swap_estimation_by_denom(
            &spot_order.order_amount,
            &spot_order.order_amount.denom,
            &spot_order.order_target_denom,
            &Decimal::zero(),
        ) {
            Ok(amm_swap_estimation) => amm_swap_estimation,
            Err(_) => {
                let mut order = spot_order.to_owned();
                order.status = Status::Canceled;
                bank_msgs.push(BankMsg::Send {
                    to_address: order.owner_address.to_string(),
                    amount: vec![order.order_amount.clone()],
                });
                PENDING_SPOT_ORDER.remove(deps.storage, order.order_id);
                SPOT_ORDER.save(deps.storage, order.order_id, &order)?;
                continue;
            }
        };

        if check_spot_order(&spot_order, &amm_swap_estimation) {
            process_spot_order(
                spot_order,
                &mut submsgs,
                env.contract.address.as_str(),
                &mut reply_info_id,
                amm_swap_estimation,
                deps.storage,
            )?;
        }
    }

    for margin_order in margin_orders.iter() {
        let mut order = margin_order.to_owned();
        let amm_swap_estimation = match querier.amm_swap_estimation_by_denom(
            &margin_order.collateral,
            &margin_order.collateral.denom,
            &margin_order.trading_asset,
            &Decimal::zero(),
        ) {
            Ok(amm_swap_estimation) => amm_swap_estimation,
            Err(_) => {
                order.status = Status::Canceled;
                PENDING_MARGIN_ORDER.remove(deps.storage, order.order_id);
                MARGIN_ORDER.save(deps.storage, order.order_id, &order)?;
                if order.order_type == MarginOrderType::LimitOpen {
                    bank_msgs.push(BankMsg::Send {
                        to_address: order.owner.to_string(),
                        amount: vec![order.collateral.clone()],
                    });
                }
                continue;
            }
        };
        if order.order_type != MarginOrderType::LimitOpen {
            match querier.mtp(order.owner.clone(), order.position_id.clone().unwrap()) {
                Ok(mtp) => match mtp.mtp {
                    Some(_) => {}
                    None => {
                        order.status = Status::Canceled;
                        PENDING_MARGIN_ORDER.remove(deps.storage, order.order_id);
                        MARGIN_ORDER.save(deps.storage, order.order_id, &order)?;
                        continue;
                    }
                },
                Err(_) => {
                    order.status = Status::Canceled;
                    PENDING_MARGIN_ORDER.remove(deps.storage, order.order_id);
                    MARGIN_ORDER.save(deps.storage, order.order_id, &order)?;
                    continue;
                }
            };
        }

        if check_margin_order(&margin_order, amm_swap_estimation) {
            process_margin_order(
                margin_order,
                &mut submsgs,
                &mut reply_info_id,
                deps.storage,
                &querier,
            )?;
        }
    }

    MAX_REPLY_ID.save(deps.storage, &reply_info_id)?;

    let resp = Response::new().add_submessages(submsgs);

    Ok(resp)
}

fn process_margin_order(
    order: &MarginOrder,
    submsgs: &mut Vec<SubMsg<ElysMsg>>,
    reply_info_id: &mut u64,
    storage: &mut dyn Storage,
    querier: &ElysQuerier<'_>,
) -> StdResult<()> {
    let (msg, reply_type) = if order.order_type == MarginOrderType::LimitOpen {
        (
            ElysMsg::margin_open_position(
                &order.owner,
                order.collateral.clone(),
                &order.trading_asset,
                order.position.clone(),
                order.leverage.clone(),
                order.take_profit_price.clone(),
            ),
            ReplyType::MarginBrokerOpen,
        )
    } else {
        let mtp = match querier
            .mtp(order.owner.clone(), order.position_id.unwrap())?
            .mtp
        {
            Some(mtp) => mtp,
            None => {
                let mut order = order.to_owned();
                order.status = Status::Canceled;
                PENDING_MARGIN_ORDER.remove(storage, order.order_id);
                MARGIN_ORDER.save(storage, order.order_id, &order)?;
                return Ok(());
            }
        };

        let amount = mtp.custody.i128();
        (
            ElysMsg::margin_close_position(&order.owner, order.position_id.unwrap(), amount),
            ReplyType::MarginBrokerClose,
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

    Ok(())
}

fn check_margin_order(
    order: &MarginOrder,
    amm_swap_estimation: AmmSwapEstimationByDenomResponse,
) -> bool {
    if order.order_type == MarginOrderType::MarketClose
        || order.order_type == MarginOrderType::MarketOpen
    {
        return false;
    }

    let trigger_price = order.trigger_price.clone().unwrap();

    let order_price = match order.collateral.denom == trigger_price.base_denom {
        true => trigger_price.rate,
        false => Decimal::one().div(trigger_price.rate),
    };

    let market_price = amm_swap_estimation.spot_price;

    match (&order.order_type, &order.position) {
        (MarginOrderType::LimitOpen, MarginPosition::Long) => market_price <= order_price,
        (MarginOrderType::LimitOpen, MarginPosition::Short) => market_price >= order_price,
        (MarginOrderType::LimitClose, MarginPosition::Long) => market_price >= order_price,
        (MarginOrderType::LimitClose, MarginPosition::Short) => market_price <= order_price,
        (MarginOrderType::StopLoss, MarginPosition::Long) => market_price <= order_price,
        (MarginOrderType::StopLoss, MarginPosition::Short) => market_price >= order_price,
        _ => false,
    }
}

fn check_spot_order(
    order: &SpotOrder,
    amm_swap_estimation: &AmmSwapEstimationByDenomResponse,
) -> bool {
    if order.order_type == SpotOrderType::MarketBuy {
        return false;
    }

    let order_price = match order.order_amount.denom == order.order_price.base_denom {
        true => order.order_price.rate,
        false => Decimal::one().div(order.order_price.rate),
    };

    let market_price = amm_swap_estimation.spot_price;

    match order.order_type {
        SpotOrderType::LimitBuy => market_price <= order_price,
        SpotOrderType::LimitSell => market_price >= order_price,
        SpotOrderType::StopLoss => market_price <= order_price,
        _ => false,
    }
}

fn process_spot_order(
    order: &SpotOrder,
    submsgs: &mut Vec<SubMsg<ElysMsg>>,
    sender: &str,
    reply_info_id: &mut u64,
    amm_swap_estimation: AmmSwapEstimationByDenomResponse,
    storage: &mut dyn Storage,
) -> StdResult<()> {
    let token_out_min_amount: Int128 = match order.order_type {
        SpotOrderType::LimitBuy => calculate_token_out_min_amount(order),
        SpotOrderType::LimitSell => calculate_token_out_min_amount(order),
        SpotOrderType::StopLoss => Int128::zero(),
        _ => Int128::zero(),
    };

    let msg = ElysMsg::amm_swap_exact_amount_in(
        sender,
        &order.order_amount,
        &amm_swap_estimation.in_route.unwrap(),
        token_out_min_amount,
        Decimal::zero(),
        &order.owner_address,
    );

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

    submsgs.push(SubMsg::reply_always(msg, *reply_info_id));

    REPLY_INFO.save(storage, *reply_info_id, &reply_info)?;

    Ok(())
}

fn calculate_token_out_min_amount(order: &SpotOrder) -> Int128 {
    let SpotOrder {
        order_amount,
        order_price,
        ..
    } = order;

    let amount = if order_amount.denom == order_price.base_denom {
        order_amount.amount * order_price.rate
    } else {
        order_amount.amount * Decimal::one().div(order_price.rate)
    };

    Int128::new((amount.u128()) as i128)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{coin, Addr, SignedDecimal, Timestamp};

    use super::*;

    #[test]
    fn test_check_spot_order_limit_buy() {
        // Arrange
        let spot_order = SpotOrder {
            order_type: SpotOrderType::LimitBuy,
            order_id: 1,
            order_price: OrderPrice {
                base_denom: "uatom".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_str("9.0").unwrap(),
            },
            order_amount: coin(1000000, "usdc"),

            owner_address: Addr::unchecked("elysd"),
            order_target_denom: "uatom".to_string(),
            status: Status::Pending,
            date: Date {
                height: 5,
                time: Timestamp::from_seconds(5),
            },
            // Initialize the rest of the SpotOrder fields here
        };
        let amm_swap_estimation = AmmSwapEstimationByDenomResponse {
            spot_price: Decimal::from_str("9.29").unwrap(),
            in_route: None,
            out_route: None,
            amount: coin(1000000, "uatom"),
            swap_fee: SignedDecimal::zero(),
            discount: SignedDecimal::zero(),
            available_liquidity: coin(1000000, "uatom"),
            weight_balance_ratio: SignedDecimal::zero(),
            price_impact: SignedDecimal::zero(),
        };

        // Act
        let result = check_spot_order(&spot_order, &amm_swap_estimation);

        // Assert
        assert_eq!(result, false); // Change as needed
    }

    #[test]
    fn test_check_spot_order_limit_sell() {
        // Arrange
        let spot_order = SpotOrder {
            order_type: SpotOrderType::LimitSell,
            order_id: 1,
            order_price: OrderPrice {
                base_denom: "uatom".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_str("10.0").unwrap(),
            },
            order_amount: coin(1000000, "uatom"),

            owner_address: Addr::unchecked("elysd"),
            order_target_denom: "usdc".to_string(),
            status: Status::Pending,
            date: Date {
                height: 5,
                time: Timestamp::from_seconds(5),
            },
            // Initialize the rest of the SpotOrder fields here
        };
        let amm_swap_estimation = AmmSwapEstimationByDenomResponse {
            spot_price: Decimal::from_str("9.29").unwrap(),
            in_route: None,
            out_route: None,
            amount: coin(1000000, "usdc"),
            swap_fee: SignedDecimal::zero(),
            discount: SignedDecimal::zero(),
            available_liquidity: coin(1000000, "usdc"),
            weight_balance_ratio: SignedDecimal::zero(),
            price_impact: SignedDecimal::zero(),
        };

        // Act
        let result = check_spot_order(&spot_order, &amm_swap_estimation);

        // Assert
        assert_eq!(result, false); // Change as needed
    }
}
