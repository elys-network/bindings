use crate::{
    msg::query_resp::GetPortfolioResp,
    states::{HISTORY, VALUE_DENOM},
    types::AccountSnapshot,
};
use cosmwasm_std::{Deps, SignedDecimal256, StdResult, Timestamp};
use cw_utils::Expiration;
use elys_bindings::ElysQuery;

pub fn get_portfolio(deps: Deps<ElysQuery>, user_address: String) -> StdResult<GetPortfolioResp> {
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let snapshots = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => snapshots,
        None => {
            return Ok(GetPortfolioResp {
                portfolio: AccountSnapshot::zero(&value_denom).portfolio,
                price_difference: SignedDecimal256::zero(),
            })
        }
    };
    let snapshot = match snapshots.last().cloned() {
        Some(expr) => expr,
        None => {
            return Ok(GetPortfolioResp {
                portfolio: AccountSnapshot::zero(&value_denom).portfolio,
                price_difference: SignedDecimal256::zero(),
            })
        }
    };

    const TWENTY_FOUR_HOURS: Expiration = Expiration::AtTime(Timestamp::from_seconds(24 * 60 * 60));

    let old_snapshot = match snapshots
        .iter()
        .filter(|snapshot| snapshot.date >= TWENTY_FOUR_HOURS)
        .last()
    {
        Some(snapshot) => snapshot,
        None => {
            return Ok(GetPortfolioResp {
                portfolio: snapshot.portfolio,
                price_difference: SignedDecimal256::zero(),
            })
        }
    };

    let actual_portfolio_balance =
        match SignedDecimal256::try_from(snapshot.portfolio.balance_usd.amount) {
            Ok(balance) => balance,
            Err(_) => {
                return Ok(GetPortfolioResp {
                    portfolio: snapshot.portfolio,
                    price_difference: SignedDecimal256::zero(),
                })
            }
        };

    let old_portfolio_balance =
        match SignedDecimal256::try_from(old_snapshot.portfolio.balance_usd.amount) {
            Ok(balance) => balance,
            Err(_) => {
                return Ok(GetPortfolioResp {
                    portfolio: snapshot.portfolio,
                    price_difference: SignedDecimal256::zero(),
                })
            }
        };

    let price_difference = actual_portfolio_balance - old_portfolio_balance;

    let resp = GetPortfolioResp {
        portfolio: snapshot.portfolio,
        price_difference,
    };
    Ok(resp)
}
