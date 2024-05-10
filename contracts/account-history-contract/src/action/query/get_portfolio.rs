use crate::{msg::query_resp::GetPortfolioResp, types::AccountSnapshotGenerator};
use cosmwasm_std::{Deps, Env, SignedDecimal256, StdResult};
use elys_bindings::{ElysQuerier, ElysQuery};

pub fn get_portfolio(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
) -> StdResult<GetPortfolioResp> {
    let querier = ElysQuerier::new(&deps.querier);

    let generator = AccountSnapshotGenerator::new(&deps)?;

    let new_snapshot =
        generator.generate_account_snapshot_for_address(&querier, &deps, &env, &user_address)?;

    let actual_portfolio_balance =
        SignedDecimal256::try_from(new_snapshot.portfolio.balance_usd).unwrap_or_default();

    let resp = GetPortfolioResp {
        portfolio: new_snapshot.portfolio,
        actual_portfolio_balance,
    };
    Ok(resp)
}
