use crate::msg::query_resp::MembershipTierResponse;
use cosmwasm_std::{Deps, Env, StdResult};
use elys_bindings::ElysQuery;

use super::user_snapshots;

pub fn get_membership_tier(
    env: Env,
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<MembershipTierResponse> {
    let user_snapshots_list = user_snapshots(env, deps, user_address)?;

    match user_snapshots_list
        .iter()
        .min_by_key(|&snapshot| snapshot.total_balance_usd)
    {
        Some(snapshot) => Ok(MembershipTierResponse::calc(
            snapshot.total_balance_usd.to_owned(),
        )),
        None => return Ok(MembershipTierResponse::zero()),
    }
}
