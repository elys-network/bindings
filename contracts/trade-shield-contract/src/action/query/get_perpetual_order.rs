use super::*;

pub fn get_perpetual_order(
    deps: Deps<ElysQuery>,
    id: u64,
) -> Result<GetPerpetualOrderResp, ContractError> {
    let order = PERPETUAL_ORDER_V2.may_load(deps.storage, id)?;

    let order = match order {
        Some(order) => order,
        None => return Err(ContractError::OrderNotFound { order_id: id }),
    };

    let order = PerpetualOrderPlus::new(order)?;
    Ok(GetPerpetualOrderResp { order })
}
