use crate::{msg::query_resp::GetTotalBalanceResp, types::AccountSnapshotGenerator};
use cosmwasm_std::{Deps, Env, StdResult};
use elys_bindings::{ElysQuerier, ElysQuery};

pub fn get_total_balance(
    deps: Deps<ElysQuery>,
    env: Env,
    user_address: String,
) -> StdResult<GetTotalBalanceResp> {
    let querier = ElysQuerier::new(&deps.querier);

    let generator = AccountSnapshotGenerator::new(&deps)?;

    let snapshot =
        generator.generate_account_snapshot_for_address(&querier, &deps, &env, &user_address)?;

    let resp = GetTotalBalanceResp {
        balances: snapshot.total_balance.clone(),
    };
    Ok(resp)
}
