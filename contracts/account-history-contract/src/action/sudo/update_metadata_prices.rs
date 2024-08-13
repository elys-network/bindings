use crate::states::METADATA;
use cosmwasm_std::{DepsMut, Response, StdResult};
use elys_bindings::{ElysMsg, ElysQuerier, ElysQuery};

pub fn update_metadata_prices(deps: DepsMut<ElysQuery>) -> StdResult<Response<ElysMsg>> {
    let querier = ElysQuerier::new(&deps.querier);

    // update metadata prices
    let mut metadata = METADATA.load(deps.storage)?;
    metadata = metadata.update_prices(&querier)?;
    METADATA.save(deps.storage, &metadata)?;

    Ok(Response::default())
}
