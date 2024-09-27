use super::*;

pub fn perpetual_get_position_for_address(
    deps: Deps<ElysQuery>,
    address: String,
    pagination: Option<PageRequest>,
) -> Result<GetPerpetualPositionsForAddressResp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let perpetual_get_position_for_address_result =
        querier.perpetual_get_position_for_address(address, pagination)?;

    let mtps = PerpetualPositionPlus::news(
        perpetual_get_position_for_address_result.get_mtp_vec(),
        deps.storage,
        &querier,
    )?;

    Ok(GetPerpetualPositionsForAddressResp {
        mtps,
        pagination: perpetual_get_position_for_address_result.pagination,
    })
}
