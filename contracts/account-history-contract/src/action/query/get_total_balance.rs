use crate::{
    msg::query_resp::GetTotalBalanceResp, states::HISTORY, types::AccountSnapshot, utils::get_today,
};
use cosmwasm_std::{Deps, Env, StdResult};
use elys_bindings::{
    query_resp::{Entry, QueryGetEntryResponse},
    ElysQuerier, ElysQuery,
};

pub fn get_total_balance(
    deps: Deps<ElysQuery>,
    env: Env,
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

    let today = get_today(&env.block);

    let snapshot = match snapshots.get(&today) {
        Some(expr) => expr,
        None => {
            return Ok(GetTotalBalanceResp {
                balances: AccountSnapshot::zero(&usdc_denom).total_balance,
            })
        }
    };

    let resp = GetTotalBalanceResp {
        balances: snapshot.total_balance.clone(),
    };
    Ok(resp)
}
