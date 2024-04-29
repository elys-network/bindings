use crate::{
    msg::query_resp::MembershipTierResponse, states::HISTORY, types::AccountSnapshotGenerator,
};
use chrono::NaiveDateTime;
use cosmwasm_std::{Decimal256, Deps, Env, StdResult, Timestamp};
use cw_utils::Expiration;
use elys_bindings::ElysQuery;

pub fn get_membership_tier(
    env: Env,
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<MembershipTierResponse> {
    let generator = AccountSnapshotGenerator::new(&deps)?;
    let expiration = match generator.expiration {
        Expiration::AtHeight(h) => Timestamp::from_seconds(h * 3), // since a block is created every 3 seconds
        Expiration::AtTime(t) => t.clone(),
        _ => panic!("never expire"),
    };
    let mut day_date = if env.block.time.seconds() < expiration.seconds() {
        Timestamp::from_seconds(0)
    } else {
        env.block
            .time
            .minus_seconds(expiration.seconds())
            .plus_days(1)
    };

    let mut user_balance_snapshots: Vec<Decimal256> = vec![];

    while day_date <= env.block.time {
        let date = NaiveDateTime::from_timestamp_opt(day_date.seconds() as i64, 0)
            .expect("Failed to convert block time to date")
            .format("%Y-%m-%d")
            .to_string();

        let day_history = HISTORY.may_load(deps.storage, &date)?;
        if let Some(day_history) = day_history {
            if let Some(portfolio) = day_history.get(&user_address) {
                user_balance_snapshots.push(portfolio.total_balance_usd)
            };
        }

        day_date = day_date.plus_days(1);
    }

    match user_balance_snapshots.iter().min_by_key(|&balance| balance) {
        Some(balance) => Ok(MembershipTierResponse::calc(balance.to_owned())),
        None => return Ok(MembershipTierResponse::zero()),
    }
}
