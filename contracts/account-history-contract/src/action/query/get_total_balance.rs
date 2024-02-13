use crate::{msg::query_resp::GetTotalBalanceResp, types::AccountSnapshotGenerator};
use cosmwasm_std::{Deps, Env, StdResult};
use elys_bindings::{account_history::types::AccountSnapshot, ElysQuerier, ElysQuery};

pub fn get_total_balance(
    deps: Deps<ElysQuery>,
    env: Env,
    user_address: String,
) -> StdResult<GetTotalBalanceResp> {
    let querier = ElysQuerier::new(&deps.querier);

    let generator = AccountSnapshotGenerator::new(&deps)?;

    let snapshot = match generator.generate_account_snapshot_for_address(
        &querier,
        &deps,
        &env,
        &user_address,
    )? {
        Some(snapshot) => snapshot,
        None => {
            return Ok(GetTotalBalanceResp {
                balances: AccountSnapshot::zero(&generator.metadata.usdc_denom).total_balance,
            })
        }
    };

    let resp = GetTotalBalanceResp {
        balances: snapshot.total_balance.clone(),
    };
    Ok(resp)
}
