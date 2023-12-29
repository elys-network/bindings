use super::*;
use crate::{bindings::{querier::ElysQuerier, query::ElysQuery}, bindings::query_resp::QueryShowCommitmentsResponse};

pub fn get_commitments(deps: Deps<ElysQuery>, delegator_address: String) -> Result<QueryShowCommitmentsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp =  querier.get_commitments(delegator_address)?;
    
    Ok(resp)
}