use super::*;
use crate::{bindings::{querier::ElysQuerier, query::ElysQuery}, bindings::query_resp::QueryDelegatorUnbondingDelegationsResponse};

pub fn get_unbonding_delegations(deps: Deps<ElysQuery>, delegator_addr: String) -> Result<QueryDelegatorUnbondingDelegationsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);
    let resp = querier.get_unbonding_delegations(delegator_addr)?;

    Ok(resp)
}