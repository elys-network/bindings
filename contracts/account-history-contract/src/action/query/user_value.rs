use crate::msg::query_resp::UserValueResponse;
use cosmwasm_std::{Deps, Env, StdError, StdResult};
use elys_bindings::ElysQuery;

use super::user_snapshots;

pub fn user_value(
    env: Env,
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<UserValueResponse> {
    let user_snapshots_list = user_snapshots(env, deps, user_address.clone())?;

    match user_snapshots_list
        .iter()
        .min_by_key(|&portfolio| portfolio.total_balance_usd)
    {
        Some(portfolio) => Ok(UserValueResponse {
            value: portfolio.to_owned(),
        }),
        None => Err(StdError::not_found(format!("user :{user_address}"))),
    }
}
