use elys_bindings::{query_resp::QueryShowCommitmentsResponse, ElysQuerier, ElysQuery};

use super::*;

pub fn get_commitments(
    deps: Deps<ElysQuery>,
    delegator_address: String,
) -> Result<QueryShowCommitmentsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp = querier.get_commitments(delegator_address)?;

    Ok(resp)
}
