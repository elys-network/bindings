use super::*;

pub fn perpetual_get_position_for_address(
    deps: Deps<ElysQuery>,
    address: String,
    pagination: Option<PageRequest>,
) -> Result<GetPerpetualPositionsForAddressResp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let PerpetualGetPositionsForAddressResponse { mtps, pagination } =
        querier.perpetual_get_position_for_address(address, pagination)?;

    let mtps = PerpetualPositionPlus::news(mtps, deps.storage, &querier)?;

    Ok(GetPerpetualPositionsForAddressResp { mtps, pagination })
}
