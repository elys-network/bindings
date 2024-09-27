use super::*;

pub fn get_perpetual_positions(
    deps: Deps<ElysQuery>,
    pagination: PageRequest,
) -> Result<GetPerpetualPositionsResp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let PerpetualQueryPositionsResponse { mtps, pagination } = querier.positions(pagination)?;

    if mtps.is_none() {
        return Ok(GetPerpetualPositionsResp {
            mtps: vec![],
            pagination,
        });
    }

    let mtps = PerpetualPositionPlus::news(
        mtps.unwrap().iter().map(|v| v.get_mtp()).collect(),
        deps.storage,
        &querier,
    )?;

    Ok(GetPerpetualPositionsResp { mtps, pagination })
}
