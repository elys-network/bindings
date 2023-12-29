use super::*;
use crate::{bindings::{querier::ElysQuerier, query::ElysQuery}, bindings::query_resp::QueryDelegatorValidatorsResponse};

pub fn get_delegator_validators(deps: Deps<ElysQuery>, delegator_address: String) -> Result<QueryDelegatorValidatorsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp = querier.get_delegator_validators(delegator_address)?;
    
    Ok(resp)
}