use crate::{
    msg::query_resp::GetPortfolioResp, states::HISTORY, types::AccountSnapshotGenerator,
    utils::get_raw_today,
};
use chrono::Days;
use cosmwasm_std::{Deps, Env, SignedDecimal256, StdResult};
use elys_bindings::{account_history::types::PortfolioBalanceSnapshot, ElysQuerier, ElysQuery};

pub fn get_portfolio(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
) -> StdResult<GetPortfolioResp> {
    let querier = ElysQuerier::new(&deps.querier);

    let generator = AccountSnapshotGenerator::new(&deps)?;

    let new_snapshot = generator
        .generate_account_snapshot_for_address(&querier, &deps, &env, &user_address)?
        .unwrap_or_default();

    let twenty_four_hours_ago = get_raw_today(&env.block)
        .checked_sub_days(Days::new(1))
        .expect("Failed to convert block time to date")
        .format("%Y-%m-%d")
        .to_string();

    let old_snapshot = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => snapshots
            .get(&twenty_four_hours_ago)
            .map_or(PortfolioBalanceSnapshot::default(), |v| v.clone()),
        None => PortfolioBalanceSnapshot::default(),
    };

    let actual_portfolio_balance =
        SignedDecimal256::try_from(new_snapshot.portfolio.balance_usd).unwrap_or_default();

    let old_portfolio_balance =
        SignedDecimal256::try_from(old_snapshot.portfolio_balance_usd).unwrap_or_default();

    let balance_24h_change = actual_portfolio_balance
        .checked_sub(old_portfolio_balance)
        .unwrap_or(actual_portfolio_balance);

    let resp = GetPortfolioResp {
        portfolio: new_snapshot.portfolio,
        actual_portfolio_balance,
        old_portfolio_balance,
        balance_24h_change,
    };
    Ok(resp)
}
