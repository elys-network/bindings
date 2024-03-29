use elys_bindings::{query_resp::QueryDelegatorValidatorsResponse, ElysQuerier, ElysQuery};

use super::*;

pub fn get_delegator_validators(
    deps: Deps<ElysQuery>,
    delegator_address: String,
) -> Result<QueryDelegatorValidatorsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp = querier.get_delegator_validators(delegator_address)?;

    Ok(resp)
}
