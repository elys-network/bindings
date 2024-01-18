use elys_bindings::{query_resp::QueryDelegatorDelegationsResponse, ElysQuerier, ElysQuery};

use super::*;

pub fn get_delegations(
    deps: Deps<ElysQuery>,
    delegator_addr: String,
) -> Result<QueryDelegatorDelegationsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);
    let resp = querier.get_delegations(delegator_addr)?;

    Ok(resp)
}
