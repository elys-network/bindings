use crate::{
    msg::query_resp::GetPortfolioResp,
    states::{HISTORY, VALUE_DENOM},
    types::Portfolio,
};
use cosmwasm_std::{DecCoin, Decimal256, Deps, StdResult};
use elys_bindings::ElysQuery;

pub fn get_pod_portfolio(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<GetPortfolioResp> {
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let snapshots = HISTORY.load(deps.storage, &user_address)?;
    let snapshot = match snapshots.last().cloned() {
        Some(expr) => expr,
        None => {
            return Ok(GetPortfolioResp {
                portfolio: Portfolio {
                    balance_usd: DecCoin::new(Decimal256::zero(), &value_denom),
                    liquid_assets_usd: DecCoin::new(Decimal256::zero(), &value_denom),
                    staked_committed_usd: DecCoin::new(Decimal256::zero(), &value_denom),
                    liquidity_positions_usd: DecCoin::new(Decimal256::zero(), &value_denom),
                    leverage_lp_usd: DecCoin::new(Decimal256::zero(), &value_denom),
                    margin_usd: DecCoin::new(Decimal256::zero(), &value_denom),
                    usdc_earn_usd: DecCoin::new(Decimal256::zero(), &value_denom),
                    borrows_usd: DecCoin::new(Decimal256::zero(), &value_denom),
                },
            })
        }
    };
    let balance_usd = DecCoin::new(
        snapshot
            .total_liquid_asset_balance
            .amount
            .checked_add(Decimal256::from(snapshot.total_staked_asset_balance))?,
        &value_denom,
    );
    let resp = GetPortfolioResp {
        portfolio: Portfolio {
            balance_usd: balance_usd,
            liquid_assets_usd: snapshot.total_liquid_asset_balance,
            staked_committed_usd: DecCoin::new(
                Decimal256::from(snapshot.total_staked_asset_balance),
                &value_denom,
            ),
            // TODO: the next fields requires to be added to the account snapshot structure first
            liquidity_positions_usd: DecCoin::new(Decimal256::zero(), &value_denom),
            leverage_lp_usd: DecCoin::new(Decimal256::zero(), &value_denom),
            margin_usd: DecCoin::new(Decimal256::zero(), &value_denom),
            usdc_earn_usd: DecCoin::new(Decimal256::zero(), &value_denom),
            borrows_usd: DecCoin::new(Decimal256::zero(), &value_denom),
        },
    };
    Ok(resp)
}
