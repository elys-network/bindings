use crate::{
    msg::query_resp::MembershipTierResponse,
    states::HISTORY,
    types::AccountSnapshotGenerator,
    utils::{get_raw_today, get_today},
};
use chrono::NaiveDateTime;
use cosmwasm_std::{Deps, Env, StdResult, Timestamp};
use cw_utils::Expiration;
use elys_bindings::{account_history::types::PortfolioBalanceSnapshot, ElysQuery};

pub fn get_membership_tier(
    env: Env,
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<MembershipTierResponse> {
    let generator = AccountSnapshotGenerator::new(&deps)?;
    let mut dates: Vec<String> = vec![];
    let expiration = match generator.expiration {
        Expiration::AtHeight(h) => Timestamp::from_seconds(h * 3), // since a block is created every 3 seconds
        Expiration::AtTime(t) => t.clone(),
        _ => panic!("never expire"),
    };
    let mut day_date = env
        .block
        .time
        .minus_seconds(expiration.seconds())
        .plus_days(1);

    while day_date < env.block.time {
        let date = NaiveDateTime::from_timestamp_opt(day_date.seconds() as i64, 0)
            .expect("Failed to convert block time to date")
            .format("%Y-%m-%d")
            .to_string();
        dates.push(date);
        day_date = day_date.plus_days(1);
    }

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
        .min_by_key(|snapshot| snapshot.total_balance_usd)
    {
        Some(snapshot) => Ok(MembershipTierResponse::calc(snapshot.total_balance_usd)),
        None => return Ok(MembershipTierResponse::zero()),
    }
}
