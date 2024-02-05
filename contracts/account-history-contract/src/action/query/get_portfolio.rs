use crate::{msg::query_resp::GetPortfolioResp, states::HISTORY, types::AccountSnapshot};
use cosmwasm_std::{Deps, SignedDecimal256, StdResult, Timestamp};
use cw_utils::Expiration;
use elys_bindings::{
    query_resp::{Entry, QueryGetEntryResponse},
    ElysQuerier, ElysQuery,
};

pub fn get_portfolio(deps: Deps<ElysQuery>, user_address: String) -> StdResult<GetPortfolioResp> {
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
    let snapshot = match snapshots.last().cloned() {
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

    let old_snapshot = match snapshots
        .iter()
        .filter(|old_snapshot| match (old_snapshot.date, snapshot.date) {
            (Expiration::AtHeight(old_time), Expiration::AtHeight(new_time)) => {
                old_time < new_time - (24 * 60 * 60 / 3)
            }
            (Expiration::AtTime(old_time), Expiration::AtTime(new_time)) => {
                if new_time < Timestamp::from_seconds(24 * 60 * 60) {
                    false
                } else {
                    old_time < new_time.minus_days(1)
                }
            }
            _ => false,
        })
        .last()
    {
        Some(snapshot) => snapshot,
        None => {
            let actual_portfolio_balance =
                match SignedDecimal256::try_from(snapshot.portfolio.balance_usd.amount) {
                    Ok(actual_portfolio_balance) => actual_portfolio_balance,
                    Err(_) => SignedDecimal256::zero(),
                };
            return Ok(GetPortfolioResp {
                portfolio: snapshot.portfolio,
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
        portfolio: snapshot.portfolio,
        actual_portfolio_balance,
        old_portfolio_balance,
        balance_24h_change,
    };
    Ok(resp)
}
