use chrono::NaiveDateTime;
use cosmwasm_std::{Deps, Env, StdResult, Timestamp};
use cw_utils::Expiration;
use elys_bindings::{account_history::types::PortfolioBalanceSnapshot, ElysQuery};

use crate::{states::HISTORY, types::AccountSnapshotGenerator};

pub fn user_snapshots(
    env: Env,
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<Vec<PortfolioBalanceSnapshot>> {
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

    let mut user_snapshots_list: Vec<PortfolioBalanceSnapshot> = vec![];

    while day_date <= env.block.time {
        let date = NaiveDateTime::from_timestamp_opt(day_date.seconds() as i64, 0)
            .expect("Failed to convert block time to date")
            .format("%Y-%m-%d")
            .to_string();
        let key = date + &user_address;

        if let Some(portfolio) = HISTORY.may_load(deps.storage, &key)? {
            user_snapshots_list.push(portfolio.to_owned())
        }

        day_date = day_date.plus_days(1);
    }

    Ok(user_snapshots_list)
}
