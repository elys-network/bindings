use crate::{
    msg::query_resp::GetTotalBalanceResp,
    states::{HISTORY, VALUE_DENOM},
    types::AccountSnapshot,
};
use cosmwasm_std::{Deps, StdResult};
use elys_bindings::ElysQuery;

pub fn get_total_balance(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<GetTotalBalanceResp> {
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let snapshots = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => snapshots,
        None => {
            return Ok(GetTotalBalanceResp {
                balances: AccountSnapshot::zero(&value_denom).total_balance,
            })
        }
    };
    let snapshot = match snapshots.last().cloned() {
        Some(expr) => expr,
        None => {
            return Ok(GetTotalBalanceResp {
                balances: AccountSnapshot::zero(&value_denom).total_balance,
            })
        }
    };
    let resp = GetTotalBalanceResp {
        balances: snapshot.total_balance,
    };
    Ok(resp)
}
