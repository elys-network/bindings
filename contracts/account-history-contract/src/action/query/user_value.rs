use crate::{
    msg::query_resp::UserValueResponse,
    states::{EXPIRATION, HISTORY},
    types::AccountSnapshot,
};
use cosmwasm_std::{BlockInfo, Deps, StdError, StdResult};
use cw_utils::Expiration;
use elys_bindings::ElysQuery;

pub fn user_value(
    deps: Deps<ElysQuery>,
    block_info: BlockInfo,
    user_address: String,
) -> StdResult<UserValueResponse> {
    let user_history = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(history) => history,
        None => return Err(StdError::not_found(format!("user :{user_address}"))),
    };

    let expiration = EXPIRATION.load(deps.storage)?;

    let clean_history: Vec<AccountSnapshot> = user_history
        .into_iter()
        .filter(|history| match (history.date, expiration) {
            (Expiration::AtHeight(time), Expiration::AtHeight(expiration)) => {
                block_info.height < time + expiration
            }
            (Expiration::AtTime(time), Expiration::AtTime(expiration)) => {
                block_info.time.nanos() < time.nanos() + expiration.nanos()
            }
            _ => false,
        })
        .collect();

    match clean_history
        .iter()
        .min_by_key(|account| account.total_liquid_asset_balance.amount)
    {
        Some(lowest_value) => Ok(UserValueResponse {
            value: lowest_value.to_owned(),
        }),
        None => Err(StdError::not_found(format!("user :{user_address}"))),
    }
}
