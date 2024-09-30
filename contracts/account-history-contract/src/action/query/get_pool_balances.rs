use crate::types::AccountSnapshotGenerator;
use cosmwasm_std::{Deps, Env, StdResult};
use elys_bindings::query_resp::QueryUserPoolResponse;
use elys_bindings::ElysQuery;

pub fn get_pool_balances(
    deps: Deps<ElysQuery>,
    address: String,
    _env: Env,
) -> StdResult<QueryUserPoolResponse> {
    let generator = AccountSnapshotGenerator::new(&deps)?;

    let pool_balances_response = generator.get_pool_balances(&deps, &address)?;

    Ok(pool_balances_response)
}
