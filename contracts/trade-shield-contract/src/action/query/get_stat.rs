use super::*;

pub fn get_stat(deps: Deps<ElysQuery>, env: Env) -> Result<GetStatResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let block_height = env.block.height;
    let AuthAddressesResponse {
        pagination:
            PageResponse {
                total: number_of_address_on_the_chain,
                ..
            },
        ..
    } = querier.accounts(Some(PageRequest {
        key: None,
        offset: None,
        limit: 1,
        count_total: true,
        reverse: true,
    }))?;
    let number_of_address_on_the_chain = number_of_address_on_the_chain.unwrap_or(0);

    let number_of_pending_order = NUMBER_OF_PENDING_ORDER.load(deps.storage)?;
    let number_of_executed_order = NUMBER_OF_EXECUTED_ORDER.load(deps.storage)?;

    let resp = GetStatResponse {
        block_height,
        number_of_address_on_the_chain,
        number_of_executed_order,
        number_of_pending_order,
        number_of_address_in_commitment: 0,
        number_of_address_in_incentive: 0,
    };

    Ok(resp)
}
