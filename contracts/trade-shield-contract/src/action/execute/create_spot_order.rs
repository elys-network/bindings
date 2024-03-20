use cosmwasm_std::{
    to_json_binary, Decimal, Int128, OverflowError, StdError, StdResult, Storage, SubMsg,
};
use elys_bindings::query_resp::AmmSwapEstimationByDenomResponse;

use crate::{helper::get_discount, msg::ReplyType};

use super::*;

pub fn create_spot_order(
    env: Env,
    deps: DepsMut<ElysQuery>,
    info: MessageInfo,
    order_type: SpotOrderType,
    order_source_denom: String,
    order_target_denom: String,
    order_price: Option<OrderPrice>,
) -> Result<Response<ElysMsg>, ContractError> {
    cw_utils::one_coin(&info)?;

    let querier = ElysQuerier::new(&deps.querier);

    if (MARKET_ORDER_ENABLED.load(deps.storage)? == false
        || SWAP_ENABLED.load(deps.storage)? == false)
        && order_type == SpotOrderType::MarketBuy
    {
        return Err(StdError::generic_err("market order is disable").into());
    }

    if let Some(price) = &order_price {
        if price.rate.is_zero() {
            return Err(StdError::generic_err("order_price: The rate cannot be zero").into());
        }
    }

    check_denom_error(
        &order_source_denom,
        &order_target_denom,
        &order_price,
        &order_type,
        &info.funds[0].denom,
    )?;

    let discount = get_discount(&deps.as_ref(), info.sender.to_string())?;

    let AmmSwapEstimationByDenomResponse { in_route, .. } = querier.amm_swap_estimation_by_denom(
        &info.funds[0],
        &order_source_denom,
        &order_target_denom,
        &discount,
    )?;
    let spot_order_max_id = SPOT_ORDER_MAX_ID.load(deps.storage)?;
    let order_id = match spot_order_max_id.checked_add(1) {
        Some(id) => id,
        None => {
            return Err(StdError::overflow(OverflowError::new(
                cosmwasm_std::OverflowOperation::Add,
                "spot_order_max_id",
                "increment one",
            ))
            .into())
        }
    };
    SPOT_ORDER_MAX_ID.save(deps.storage, &order_id)?;

    let new_order: SpotOrder = SpotOrder::new(
        order_id,
        order_type.clone(),
        order_price,
        info.funds[0].clone(),
        info.sender.clone(),
        order_target_denom,
        &env.block,
    );

    let resp = create_resp(
        env.contract.address.as_str(),
        &new_order,
        deps.storage,
        discount,
        in_route.unwrap(),
    )?;

    SPOT_ORDER.save(deps.storage, new_order.order_id, &new_order)?;
    if new_order.order_type != SpotOrderType::MarketBuy {
        PENDING_SPOT_ORDER.save(deps.storage, new_order.order_id, &new_order)?;
    }
    let mut ids = USER_SPOT_ORDER
        .may_load(deps.storage, new_order.owner_address.as_str())?
        .unwrap_or(vec![]);
    ids.push(new_order.order_id);
    USER_SPOT_ORDER.save(deps.storage, new_order.owner_address.as_str(), &ids)?;
    Ok(resp)
}

fn check_denom_error(
    order_source_denom: &str,
    order_target_denom: &str,
    order_price: &Option<OrderPrice>,
    order_type: &SpotOrderType,
    funds_send_denom: &str,
) -> Result<(), ContractError> {
    if order_source_denom != funds_send_denom {
        return Err(ContractError::SpotOrderWrongFund);
    }

    if order_source_denom == order_target_denom {
        return Err(ContractError::SpotOrderSameDenom);
    }

    if order_type == &SpotOrderType::MarketBuy {
        return Ok(());
    }

    if order_price.is_none() {
        return Err(StdError::not_found("order price").into());
    }

    let order_price = order_price.clone().unwrap();

    if order_price.base_denom != order_source_denom {
        return Err(StdError::generic_err("fund denom should be base denom").into());
    }
    if order_price.quote_denom != order_target_denom {
        return Err(StdError::generic_err("target denom should be quote denom").into());
    }

    Ok(())
}

fn create_resp(
    sender: &str,
    new_order: &SpotOrder,
    storage: &mut dyn Storage,
    discount: Decimal,
    in_route: Vec<SwapAmountInRoute>,
) -> StdResult<Response<ElysMsg>> {
    let resp = Response::new().add_event(
        Event::new("create_spot_order").add_attribute("order_id", new_order.order_id.to_string()),
    );
    if new_order.order_type != SpotOrderType::MarketBuy {
        return Ok(resp);
    }

    let reply_info_max_id = MAX_REPLY_ID.load(storage)?;

    let reply_id = match reply_info_max_id.checked_add(1) {
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

    MAX_REPLY_ID.save(storage, &reply_id)?;

    let swap_msg = ElysMsg::amm_swap_exact_amount_in(
        sender,
        &new_order.order_amount,
        &in_route,
        Int128::zero(),
        discount,
        &new_order.owner_address,
    );

    let reply_info = ReplyInfo {
        id: reply_id,
        reply_type: ReplyType::SpotOrderMarketBuy,
        data: Some(to_json_binary(&new_order.order_id)?),
    };

    REPLY_INFO.save(storage, reply_id, &reply_info)?;

    let sub_msg = SubMsg::reply_always(swap_msg, reply_id);

    Ok(resp.add_submessage(sub_msg))
}
