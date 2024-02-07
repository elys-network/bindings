use core::panic;

use crate::{
    msg::query_resp::GetPortfolioResp,
    states::HISTORY,
    types::AccountSnapshot,
    utils::{get_raw_today, get_today},
};
use chrono::Days;
use cosmwasm_std::{Deps, Env, SignedDecimal256, StdResult};
use elys_bindings::{
    query_resp::{Entry, QueryGetEntryResponse},
    ElysQuerier, ElysQuery,
};

pub fn get_portfolio(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
) -> StdResult<GetPortfolioResp> {
    let querier = ElysQuerier::new(&deps.querier);
    let QueryGetEntryResponse {
        entry: Entry {
            denom: usdc_denom, ..
        },
    } = querier.get_asset_profile("uusdc".to_string())?;
    let snapshots = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => snapshots,
        None => {
            return Ok(GetPortfolioResp {
                portfolio: AccountSnapshot::zero(&usdc_denom).portfolio,
                actual_portfolio_balance: SignedDecimal256::zero(),
                old_portfolio_balance: SignedDecimal256::zero(),
                balance_24h_change: SignedDecimal256::zero(),
            })
        }
    };

    let today = get_today(&env.block);

    let snapshot = match snapshots.get(&today) {
        Some(expr) => expr,
        None => {
            return Ok(GetPortfolioResp {
                portfolio: AccountSnapshot::zero(&usdc_denom).portfolio,
                actual_portfolio_balance: SignedDecimal256::zero(),
                old_portfolio_balance: SignedDecimal256::zero(),
                balance_24h_change: SignedDecimal256::zero(),
            })
        }
    };

    let twenty_four_hours_ago = match get_raw_today(&env.block).checked_sub_days(Days::new(1)) {
        Some(date_time) => date_time.format("%Y-%m-%d").to_string(),
        None => panic!("Failed to convert block time to date"),
    };

    let old_snapshot = match snapshots.get(&twenty_four_hours_ago) {
        Some(snapshot) => snapshot,
        None => {
            let actual_portfolio_balance =
                match SignedDecimal256::try_from(snapshot.portfolio.balance_usd.amount) {
                    Ok(actual_portfolio_balance) => actual_portfolio_balance,
                    Err(_) => SignedDecimal256::zero(),
                };
            return Ok(GetPortfolioResp {
                portfolio: snapshot.portfolio.clone(),
                actual_portfolio_balance,
                old_portfolio_balance: SignedDecimal256::zero(),
                balance_24h_change: SignedDecimal256::zero(),
            });
        }
    };

    let actual_portfolio_balance =
        match SignedDecimal256::try_from(snapshot.portfolio.balance_usd.amount) {
            Ok(balance) => balance,
            Err(_) => SignedDecimal256::zero(),
        };

    let old_portfolio_balance =
        match SignedDecimal256::try_from(old_snapshot.portfolio.balance_usd.amount) {
            Ok(balance) => balance,
            Err(_) => SignedDecimal256::zero(),
        };

    let balance_24h_change = actual_portfolio_balance - old_portfolio_balance;

    let resp = GetPortfolioResp {
        portfolio: snapshot.portfolio.clone(),
        actual_portfolio_balance,
        old_portfolio_balance,
        balance_24h_change,
    };
    Ok(resp)
}
