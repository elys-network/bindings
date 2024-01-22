use crate::{
    msg::query_resp::GetPortfolioResp,
    states::{HISTORY, VALUE_DENOM},
    types::AccountSnapshot,
};
use cosmwasm_std::{Deps, StdResult};
use elys_bindings::ElysQuery;

pub fn get_portfolio(deps: Deps<ElysQuery>, user_address: String) -> StdResult<GetPortfolioResp> {
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let snapshots = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => snapshots,
        None => {
            return Ok(GetPortfolioResp {
                portfolio: AccountSnapshot::zero(&value_denom).portfolio,
            })
        }
    };
    let snapshot = match snapshots.last().cloned() {
        Some(expr) => expr,
        None => {
            return Ok(GetPortfolioResp {
                portfolio: AccountSnapshot::zero(&value_denom).portfolio,
            })
        }
    };
    let resp = GetPortfolioResp {
        portfolio: snapshot.portfolio,
    };
    Ok(resp)
}
