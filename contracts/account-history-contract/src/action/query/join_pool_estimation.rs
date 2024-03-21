use cosmwasm_std::{Deps, StdResult, Coin};
use elys_bindings::{
    query_resp::QueryJoinPoolEstimationResponse, ElysQuerier,
    ElysQuery,
};

pub fn join_pool_estimation(
    deps: Deps<ElysQuery>,
    pool_id: u64,
    amounts_in: Vec<Coin>
) -> StdResult<QueryJoinPoolEstimationResponse> {
    let querier = ElysQuerier::new(&deps.querier);
    let resp = querier.join_pool_estimation(pool_id, amounts_in)?;
    Ok(resp)
}
