use super::*;

pub fn get_margin_positions(
    deps: Deps<ElysQuery>,
    pagination: PageRequest,
) -> Result<GetMarginPositionsResp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let MarginQueryPositionsResponse { mtps, pagination } = querier.positions(pagination)?;

    if mtps.is_none() {
        return Ok(GetMarginPositionsResp {
            mtps: vec![],
            pagination,
        });
    }

    let mtps = MarginPositionPlus::news(mtps.unwrap(), deps.storage, &querier)?;

    Ok(GetMarginPositionsResp { mtps, pagination })
}
