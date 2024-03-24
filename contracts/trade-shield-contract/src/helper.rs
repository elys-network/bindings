use cosmwasm_std::{
    from_json, Decimal, QuerierWrapper, Response, StdResult, Storage, SubMsgResult,
};
use elys_bindings::account_history::msg::query_resp::MembershipTierResponse;
use elys_bindings::account_history::msg::QueryMsg as AccountHistoryQueryMsg;
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

pub fn get_discount(
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
