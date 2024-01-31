use super::*;

pub fn get_perpetual_order(
    deps: Deps<ElysQuery>,
    id: u64,
) -> Result<GetPerpetualOrderResp, ContractError> {
    let order = PERPETUAL_ORDER.may_load(deps.storage, id)?;

    match order {
        Some(order) => Ok(GetPerpetualOrderResp { order }),
        None => Err(ContractError::OrderNotFound { order_id: id }),
    }
}
