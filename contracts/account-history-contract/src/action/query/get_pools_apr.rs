use cosmwasm_std::{Deps, StdResult};
use elys_bindings::{query_resp::QueryIncentivePoolAprsResponse, ElysQuerier, ElysQuery};

pub fn get_pools_apr(
    deps: Deps<ElysQuery>,
    pool_ids: Option<Vec<u64>>,
) -> StdResult<QueryIncentivePoolAprsResponse> {
    let querier = ElysQuerier::new(&deps.querier);
    let resp = querier.get_pools_apr(pool_ids)?;
    Ok(resp)
}
