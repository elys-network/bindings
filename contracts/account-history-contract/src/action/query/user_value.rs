use crate::{msg::query_resp::UserValueResponse, states::HISTORY, types::AccountSnapshotGenerator};
use chrono::NaiveDateTime;
use cosmwasm_std::{Deps, Env, StdError, StdResult, Timestamp};
use cw_utils::Expiration;
use elys_bindings::{account_history::types::PortfolioBalanceSnapshot, ElysQuery};

pub fn user_value(env: Env, deps: Deps<ElysQuery>, user_address: String) -> StdResult<UserValueResponse> {
    let generator = AccountSnapshotGenerator::new(&deps)?;
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
    let mut user_balance_snapshots: Vec<PortfolioBalanceSnapshot> = vec![];

    while day_date < env.block.time {
        let date = NaiveDateTime::from_timestamp_opt(day_date.seconds() as i64, 0)
            .expect("Failed to convert block time to date")
            .format("%Y-%m-%d")
            .to_string();

        let day_history = HISTORY.may_load(deps.storage, &date)?;
        if let Some(day_history) = day_history {
            if let Some(portfolio) = day_history.get(&user_address) {
                user_balance_snapshots.push(portfolio.clone())
            };
        }

        day_date = day_date.plus_days(1);
    }

    match user_balance_snapshots.iter().min_by_key(|&portfolio| portfolio.total_balance_usd) {
        Some(portfolio) => Ok(UserValueResponse { value: portfolio.to_owned()}),
        None => Err(StdError::not_found(format!("user :{user_address}"))),
    }
    
}
