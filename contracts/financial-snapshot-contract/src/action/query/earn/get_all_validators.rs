use super::*;
use crate::{bindings::{querier::ElysQuerier, query::ElysQuery}, bindings::query_resp::QueryDelegatorValidatorsResponse};

pub fn get_all_validators(deps: Deps<ElysQuery>, delegator_address: Option<String>) -> Result<QueryDelegatorValidatorsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp = match delegator_address {
        Some(address) => querier.get_all_validators(address)?,
        None => querier.get_all_validators("".to_string())?,
    };
    
    Ok(resp)
}