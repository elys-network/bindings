use cosmwasm_std::{
    from_json, Decimal, Deps, QuerierWrapper, Response, StdResult, Storage, SubMsgResult,
};
use elys_bindings::account_history::msg::query_resp::MembershipTierResponse;
use elys_bindings::account_history::msg::QueryMsg as AccountHistoryQueryMsg;
use elys_bindings::trade_shield::states::{
    PENDING_PERPETUAL_ORDER, PENDING_SPOT_ORDER, PERPETUAL_ORDER, SORTED_PENDING_PERPETUAL_ORDER,
    SORTED_PENDING_SPOT_ORDER, SPOT_ORDER,
};
use elys_bindings::trade_shield::types::{PerpetualOrder, SpotOrder, Status};
use elys_bindings::ElysQuery;
use elys_bindings::{trade_shield::states::ACCOUNT_HISTORY_ADDRESS, ElysMsg};

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

pub fn get_mut_discount(
    storage: &mut dyn Storage,
    querier: QuerierWrapper<'_, ElysQuery>,
    user_address: String,
) -> StdResult<Decimal> {
    let account_history_address = match ACCOUNT_HISTORY_ADDRESS.load(storage)? {
        Some(account_history_address) => account_history_address,
        None => return Ok(Decimal::zero()),
    };

    let discount = match querier.query_wasm_smart::<MembershipTierResponse>(
        &account_history_address,
        &AccountHistoryQueryMsg::GetMembershipTier { user_address },
    ) {
        Ok(resp) => resp.discount,
        Err(_) => Decimal::zero(),
    };

    Ok(discount)
}

pub fn get_discount(deps: &Deps<ElysQuery>, user_address: String) -> StdResult<Decimal> {
    let account_history_address = match ACCOUNT_HISTORY_ADDRESS.load(deps.storage)? {
        Some(account_history_address) => account_history_address,
        None => return Ok(Decimal::zero()),
    };

    let discount = match deps.querier.query_wasm_smart::<MembershipTierResponse>(
        &account_history_address,
        &AccountHistoryQueryMsg::GetMembershipTier { user_address },
    ) {
        Ok(resp) => resp.discount,
        Err(_) => Decimal::zero(),
    };

    Ok(discount)
}

pub fn remove_spot_order(
    order_id: u64,
    new_status: Status,
    storage: &mut dyn Storage,
) -> StdResult<()> {
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
    Ok(())
}

pub fn remove_perpetual_order(
    order_id: u64,
    new_status: Status,
    storage: &mut dyn Storage,
) -> StdResult<()> {
    let mut order = PENDING_PERPETUAL_ORDER.load(storage, order_id)?;
    let key = order.gen_key()?;
    let mut vec: Vec<u64> = SORTED_PENDING_PERPETUAL_ORDER.load(storage, key.as_str())?;
    let mut index = PerpetualOrder::binary_search(&order.trigger_price, storage, &vec)?;
    let size_of_vec = vec.len();
    while vec[index] != order_id && index < size_of_vec {
        index += 1;
    }
    if index >= size_of_vec {
        return Err(cosmwasm_std::StdError::generic_err("overflow error"));
    }
    vec.remove(index);
    SORTED_PENDING_PERPETUAL_ORDER.save(storage, key.as_str(), &vec)?;
    order.status = new_status;
    PERPETUAL_ORDER.save(storage, order.order_id, &order)?;
    PENDING_PERPETUAL_ORDER.remove(storage, order.order_id);
    Ok(())
}
