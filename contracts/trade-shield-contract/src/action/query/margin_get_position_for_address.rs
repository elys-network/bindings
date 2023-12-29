use super::*;

pub fn margin_get_position_for_address(
    deps: Deps<ElysQuery>,
    address: String,
    pagination: PageRequest,
) -> Result<GetMarginPositionsForAddressResp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let MarginGetPositionsForAddressResponse { mtps, pagination } =
        querier.margin_get_position_for_address(address, pagination)?;

    let mtps = MarginPositionPlus::news(mtps, deps.storage, &querier)?;

    Ok(GetMarginPositionsForAddressResp { mtps, pagination })
}
