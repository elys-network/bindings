use super::*;

pub fn get_stat(deps: Deps<ElysQuery>, env: Env) -> Result<GetStatResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let _height = env.block.height;
    let AuthAddressesResponse {
        pagination:
            PageResponse {
                total: _number_of_address_on_the_chain,
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

    let _number_of_pending_order = NUMBER_OF_PENDING_ORDER.load(deps.storage)?;
    let _number_of_executed_order = NUMBER_OF_EXECUTED_ORDER.load(deps.storage)?;

    unimplemented!()
}
