use crate::{
    msg::query_resp::MembershipTierResponse,
    states::{EXPIRATION, HISTORY},
    types::AccountSnapshot,
};
use cosmwasm_std::{BlockInfo, Deps, StdResult};
use cw_utils::Expiration;
use elys_bindings::ElysQuery;

pub fn get_membership_tier(
    deps: Deps<ElysQuery>,
    block_info: BlockInfo,
    user_address: String,
) -> StdResult<MembershipTierResponse> {
    let user_history = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(history) => history,
        None => return Ok(MembershipTierResponse::zero()),
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
        .min_by_key(|account| account.total_balance.total_balance.amount)
    {
        Some(snapshot) => Ok(MembershipTierResponse::calc(
            snapshot.total_balance.total_balance.amount,
        )),
        None => return Ok(MembershipTierResponse::zero()),
    }
}
