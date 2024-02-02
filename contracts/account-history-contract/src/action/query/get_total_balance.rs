use crate::{msg::query_resp::GetTotalBalanceResp, states::HISTORY, types::AccountSnapshot};
use cosmwasm_std::{Deps, StdResult};
use elys_bindings::{
    query_resp::{Entry, QueryGetEntryResponse},
    ElysQuerier, ElysQuery,
};

pub fn get_total_balance(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<GetTotalBalanceResp> {
    let querier = ElysQuerier::new(&deps.querier);
    let QueryGetEntryResponse {
        entry: Entry {
            denom: usdc_denom, ..
        },
    } = querier.get_asset_profile("uusdc".to_string())?;

    let snapshots = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => snapshots,
        None => {
            return Ok(GetTotalBalanceResp {
                balances: AccountSnapshot::zero(&usdc_denom).total_balance,
            })
        }
    };
    let snapshot = match snapshots.last().cloned() {
        Some(expr) => expr,
        None => {
            return Ok(GetTotalBalanceResp {
                balances: AccountSnapshot::zero(&usdc_denom).total_balance,
            })
        }
    };
    let resp = GetTotalBalanceResp {
        balances: snapshot.total_balance,
    };
    Ok(resp)
}
