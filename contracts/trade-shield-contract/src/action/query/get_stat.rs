use super::*;

pub fn get_stat(deps: Deps<ElysQuery>, env: Env) -> Result<GetStatResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let block_height = env.block.height;

    let number_of_pending_order = NUMBER_OF_PENDING_ORDER.load(deps.storage)?;
    let number_of_executed_order = NUMBER_OF_EXECUTED_ORDER.load(deps.storage)?;

    let commitment_number_of_commitments_response = querier.commitment_number_of_commitments()?;
    let number_of_address_in_commitment = commitment_number_of_commitments_response.number as u64;

    let resp = GetStatResponse {
        block_height,
        number_of_executed_order,
        number_of_pending_order,
        number_of_address_in_commitment,
    };

    Ok(resp)
}
