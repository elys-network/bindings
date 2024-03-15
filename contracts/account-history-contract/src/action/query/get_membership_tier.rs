use crate::{msg::query_resp::MembershipTierResponse, states::HISTORY};
use cosmwasm_std::{Deps, StdResult};
use elys_bindings::{account_history::types::PortfolioBalanceSnapshot, ElysQuery};

pub fn get_membership_tier(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<MembershipTierResponse> {
    let user_history: Vec<PortfolioBalanceSnapshot> =
        match HISTORY.may_load(deps.storage, &user_address)? {
            Some(history) => history,
            None => return Ok(MembershipTierResponse::zero()),
        }
        .values()
        .cloned()
        .collect();

    match user_history
        .iter()
        .min_by_key(|snapshot| snapshot.total_balance_usd.amount)
    {
        Some(snapshot) => Ok(MembershipTierResponse::calc(
            snapshot.total_balance_usd.amount,
        )),
        None => return Ok(MembershipTierResponse::zero()),
    }
}
