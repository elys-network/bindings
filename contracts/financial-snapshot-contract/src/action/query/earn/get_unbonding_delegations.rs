use elys_bindings::{
    query_resp::QueryDelegatorUnbondingDelegationsResponse, ElysQuerier, ElysQuery,
};

use super::*;

pub fn get_unbonding_delegations(
    deps: Deps<ElysQuery>,
    delegator_addr: String,
) -> Result<QueryDelegatorUnbondingDelegationsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);
    let resp = querier.get_unbonding_delegations(delegator_addr)?;

    Ok(resp)
}
