use crate::{msg::query_resp::UserValueResponse, states::HISTORY};
use cosmwasm_std::{Deps, StdError, StdResult};
use elys_bindings::{account_history::types::AccountSnapshot, ElysQuery};

pub fn user_value(deps: Deps<ElysQuery>, user_address: String) -> StdResult<UserValueResponse> {
    let user_history: Vec<AccountSnapshot> = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(history) => history,
        None => return Err(StdError::not_found(format!("user :{user_address}"))),
    }
    .values()
    .cloned()
    .collect();

    match user_history
        .iter()
        .min_by_key(|account| account.total_balance.total_balance.amount)
    {
        Some(lowest_value) => Ok(UserValueResponse {
            value: lowest_value.to_owned(),
        }),
        None => Err(StdError::not_found(format!("user :{user_address}"))),
    }
}
