use super::*;
use cosmwasm_std::{from_json, Binary, DepsMut, StdError, SubMsgResult};

pub fn reply_to_spot_order_market(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    if let Err(err) = module_resp.into_result() {
        return Err(StdError::generic_err(format!("{}: {}: {}", file!(), line!(), err)).into());
    };

    let order_id: u64 = match data {
        Some(order_id) => from_json(&order_id)
            .map_err(|e| StdError::generic_err(format!("{}: {}: {}", file!(), line!(), e)))?,
        None => {
            return Err(
                StdError::generic_err(format!("{}: {}: no meta_data", file!(), line!())).into(),
            )
        }
    };

    let mut order = SPOT_ORDER
        .load(deps.storage, order_id)
        .map_err(|e| StdError::generic_err(format!("{}: {}: {}", file!(), line!(), e)))?;

    order.status = Status::Executed;

    SPOT_ORDER
        .save(deps.storage, order_id, &order)
        .map_err(|e| StdError::generic_err(format!("{}: {}: {}", file!(), line!(), e)))?;

    let resp: Response<ElysMsg> = Response::new().add_event(
        Event::new("reply_to_spot_order_market").add_attribute("order_id", order_id.to_string()),
    );

    Ok(resp)
}
