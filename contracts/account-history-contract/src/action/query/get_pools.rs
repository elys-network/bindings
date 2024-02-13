use cosmwasm_std::{Deps, StdResult};
use elys_bindings::{
    query_resp::PoolFilterType, query_resp::QueryEarnPoolResponse, types::PageRequest, ElysQuerier,
    ElysQuery,
};

pub fn get_pools(
    deps: Deps<ElysQuery>,
    pool_ids: Option<Vec<u64>>,
    filter_type: PoolFilterType,
    pagination: Option<PageRequest>,
) -> StdResult<QueryEarnPoolResponse> {
    let querier = ElysQuerier::new(&deps.querier);
    let resp = querier.get_all_pools(pool_ids, filter_type as i32, pagination)?;
    Ok(resp)
}
