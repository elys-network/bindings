use crate::{
    states::HISTORY,
    msg::query_resp::UserRewardsResponse,};
use elys_bindings::ElysQuery;
use cosmwasm_std::{Deps, StdError, StdResult};

pub fn user_rewards(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<UserRewardsResponse> {
    let user_history: Vec<crate::types::AccountSnapshot> = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(history) => history,
        None => return Err(StdError::not_found(format!("user :{user_address}"))),
    };

    let latest_snapshot = user_history.last().unwrap();
    Ok(UserRewardsResponse {
        rewards: latest_snapshot.rewards.to_owned(),
    })
}