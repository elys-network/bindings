use std::str::FromStr;

use cosmwasm_std::{
    from_json, BankMsg, Decimal, OverflowError, OverflowOperation, QuerierWrapper, Response,
    StdError, StdResult, Storage, SubMsgResult, Uint128,
};
use elys_bindings::trade_shield::states::{
    NUMBER_OF_EXECUTED_ORDER, NUMBER_OF_PENDING_ORDER, PENDING_PERPETUAL_ORDER_V2,
    PENDING_SPOT_ORDER, PERPETUAL_ORDER_V2, SORTED_PENDING_PERPETUAL_ORDER,
    SORTED_PENDING_SPOT_ORDER, SPOT_ORDER,
};
use elys_bindings::trade_shield::types::{PerpetualOrderType, PerpetualOrderV2, SpotOrder, Status};
use elys_bindings::ElysMsg;
use elys_bindings::{ElysQuerier, ElysQuery};

use serde::de::DeserializeOwned;

pub fn get_response_from_reply<T: DeserializeOwned>(
    module_resp: SubMsgResult,
) -> Result<T, Response<ElysMsg>> {
    let response = match module_resp.into_result() {
        Ok(response) => response,
        Err(err) => return Err(Response::new().add_attribute("error", err)),
    };

    let data = match response.data {
        Some(data) => data,
        None => return Err(Response::new().add_attribute("error", "No Data")),
    };

    match from_json::<T>(&data) {
        Ok(resp) => Ok(resp),
        Err(err) => Err(Response::new().add_attribute("error", err.to_string())),
    }
}

pub fn get_discount(
    querier: QuerierWrapper<'_, ElysQuery>,
    user_address: String,
) -> StdResult<Decimal> {
    let querier = ElysQuerier::new(&querier);
    let mut discount_str = match querier.tier_calculate_discount(user_address.clone()) {
        Ok(resp) => resp.discount,
        Err(_) => "0".to_string(),
    };

    if user_address == "elys1u8c28343vvhwgwhf29w6hlcz73hvq7lwxmrl46" {
        discount_str = "20".to_string()
    }

    let val = Uint128::from_str(&discount_str)?;
    let discount_str = match Decimal::from_atomics(val, 2) {
        Ok(resp) => resp,
        Err(_) => Decimal::zero(),
    };
    Ok(discount_str)
}

pub fn remove_spot_order(
    order_id: u64,
    new_status: Status,
    storage: &mut dyn Storage,
) -> StdResult<Option<BankMsg>> {
    let mut order = PENDING_SPOT_ORDER.load(storage, order_id)?;
    let key = order.gen_key()?;
    let mut vec: Vec<u64> = SORTED_PENDING_SPOT_ORDER.load(storage, key.as_str())?;
    let mut index = SpotOrder::binary_search(&order.order_price.rate, storage, &vec)?;
    let size_of_vec = vec.len();
    while vec[index] != order_id && index < size_of_vec {
        index += 1;
    }
    if index >= size_of_vec {
        return Err(cosmwasm_std::StdError::generic_err("overflow error"));
    }
    vec.remove(index);
    SORTED_PENDING_SPOT_ORDER.save(storage, key.as_str(), &vec)?;
    order.status = new_status;
    SPOT_ORDER.save(storage, order.order_id, &order)?;
    PENDING_SPOT_ORDER.remove(storage, order.order_id);
    change_the_number_of_order(storage, &order.status)?;
    let bank_msg = if order.status == Status::Canceled {
        Some(BankMsg::Send {
            to_address: order.owner_address.to_string(),
            amount: vec![order.order_amount.clone()],
        })
    } else {
        None
    };
    Ok(bank_msg)
}

pub fn remove_perpetual_order(
    order_id: u64,
    new_status: Status,
    storage: &mut dyn Storage,
    position_id: Option<u64>,
) -> StdResult<Option<BankMsg>> {
    let mut order = PENDING_PERPETUAL_ORDER_V2.load(storage, order_id).unwrap();
    let key = order.gen_key()?;
    let mut vec: Vec<u64> = SORTED_PENDING_PERPETUAL_ORDER.load(storage, key.as_str())?;
    let mut index = PerpetualOrderV2::binary_search(&order.trigger_price, storage, &vec)?;
    let size_of_vec = vec.len();
    while vec[index] != order_id && index < size_of_vec {
        index += 1;
    }
    if index >= size_of_vec {
        return Err(cosmwasm_std::StdError::generic_err("overflow error"));
    }
    if new_status == Status::Executed {
        order.position_id = position_id
    }
    vec.remove(index);
    SORTED_PENDING_PERPETUAL_ORDER.save(storage, key.as_str(), &vec)?;
    order.status = new_status;
    PERPETUAL_ORDER_V2.save(storage, order.order_id, &order)?;
    PENDING_PERPETUAL_ORDER_V2.remove(storage, order.order_id);
    change_the_number_of_order(storage, &order.status)?;
    let bank_msg =
        if order.status == Status::Canceled && order.order_type == PerpetualOrderType::LimitOpen {
            Some(BankMsg::Send {
                to_address: order.owner.to_string(),
                amount: vec![order.collateral.clone()],
            })
        } else {
            None
        };
    Ok(bank_msg)
}

fn change_the_number_of_order(storage: &mut dyn Storage, status: &Status) -> StdResult<()> {
    let number_of_pending_order = match NUMBER_OF_PENDING_ORDER.load(storage)?.checked_sub(1) {
        Some(number) => Ok(number),
        None => Err(StdError::overflow(OverflowError::new(
            OverflowOperation::Sub,
            "number_of_pending_order",
            1,
        ))),
    }?;
    NUMBER_OF_PENDING_ORDER.save(storage, &number_of_pending_order)?;
    if status == &Status::Executed {
        let number_of_executed_order = match NUMBER_OF_EXECUTED_ORDER.load(storage)?.checked_add(1)
        {
            Some(number) => Ok(number),
            None => Err(StdError::overflow(OverflowError::new(
                OverflowOperation::Add,
                "number_of_pending_order",
                1,
            ))),
        }?;
        NUMBER_OF_EXECUTED_ORDER.save(storage, &number_of_executed_order)?;
    };
    Ok(())
}
