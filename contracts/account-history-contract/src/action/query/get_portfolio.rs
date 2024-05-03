use crate::{
    msg::query_resp::GetPortfolioResp, states::HISTORY, types::AccountSnapshotGenerator,
    utils::get_raw_today,
};
use chrono::Days;
use cosmwasm_std::{Deps, Env, SignedDecimal256, StdResult};
use cw_utils::Expiration;
use elys_bindings::{account_history::types::PortfolioBalanceSnapshot, ElysQuerier, ElysQuery};

pub fn get_portfolio(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
) -> StdResult<GetPortfolioResp> {
    let querier = ElysQuerier::new(&deps.querier);

    let generator = AccountSnapshotGenerator::new(&deps)?;

    let new_snapshot =
        generator.generate_account_snapshot_for_address(&querier, &deps, &env, &user_address)?;

    let twenty_four_hours_ago = get_raw_today(&env.block)
        .checked_sub_days(Days::new(1))
        .expect("Failed to convert block time to date ")
        .format("%Y-%m-%d")
        .to_string();

    let old_snapshot = HISTORY
        .may_load(deps.storage, &twenty_four_hours_ago)
        .map_or(
            PortfolioBalanceSnapshot::default(),
            |snapshots| -> PortfolioBalanceSnapshot {
                snapshots
                    .unwrap_or_default()
                    .get(&user_address)
                    .unwrap_or(&PortfolioBalanceSnapshot::default())
                    .clone()
            },
        );

    let actual_portfolio_balance =
        SignedDecimal256::try_from(new_snapshot.portfolio.balance_usd).unwrap_or_default();

    let old_portfolio_balance =
        SignedDecimal256::try_from(old_snapshot.portfolio_balance_usd).unwrap_or_default();

    let balance_24h_change = if (old_snapshot.date == Expiration::Never {}) {
        SignedDecimal256::zero()
    } else {
        actual_portfolio_balance
            .clone()
            .checked_sub(old_portfolio_balance)
            .unwrap_or_default()
    };

    let resp = GetPortfolioResp {
        portfolio: new_snapshot.portfolio,
        actual_portfolio_balance,
        old_portfolio_balance,
        balance_24h_change,
    };
    Ok(resp)
}
