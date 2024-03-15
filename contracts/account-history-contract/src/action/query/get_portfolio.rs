use core::panic;

use crate::{
    msg::query_resp::GetPortfolioResp, states::HISTORY, types::AccountSnapshotGenerator,
    utils::get_raw_today,
};
use chrono::Days;
use cosmwasm_std::{Deps, Env, SignedDecimal256, StdResult};
use elys_bindings::{
    account_history::types::{AccountSnapshot, PortfolioBalanceSnapshot},
    ElysQuerier, ElysQuery,
};

pub fn get_portfolio(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
) -> StdResult<GetPortfolioResp> {
    let querier = ElysQuerier::new(&deps.querier);

    let generator = AccountSnapshotGenerator::new(&deps)?;

    let new_snapshot = match generator.generate_account_snapshot_for_address(
        &querier,
        &deps,
        &env,
        &user_address,
    )? {
        Some(snapshot) => snapshot,
        None => {
            return Ok(GetPortfolioResp {
                portfolio: AccountSnapshot::zero(&generator.metadata.usdc_denom).portfolio,
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

    let old_snapshot = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => match snapshots.get(&twenty_four_hours_ago) {
            Some(snapshot) => snapshot.clone(),
            None => PortfolioBalanceSnapshot::default(),
        },
        None => PortfolioBalanceSnapshot::default(),
    };

    let actual_portfolio_balance =
        match SignedDecimal256::try_from(new_snapshot.portfolio.balance_usd.amount) {
            Ok(balance) => balance,
            Err(_) => SignedDecimal256::zero(),
        };

    let old_portfolio_balance =
        match SignedDecimal256::try_from(old_snapshot.portfolio_balance_usd.amount) {
            Ok(balance) => balance,
            Err(_) => SignedDecimal256::zero(),
        };

    let balance_24h_change = match actual_portfolio_balance
        .clone()
        .checked_sub(old_portfolio_balance)
    {
        Ok(balance_24h_change) => balance_24h_change,
        Err(_) => actual_portfolio_balance,
    };

    let resp = GetPortfolioResp {
        portfolio: new_snapshot.portfolio.clone(),
        actual_portfolio_balance,
        old_portfolio_balance,
        balance_24h_change,
    };
    Ok(resp)
}
