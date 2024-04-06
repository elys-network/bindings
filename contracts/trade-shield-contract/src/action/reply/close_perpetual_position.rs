use cosmwasm_std::{from_json, Binary, StdError, SubMsgResult};

use crate::helper::get_response_from_reply;

use super::*;

pub fn reply_to_close_perpetual_order(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let order_id: u64 = from_json(&data.unwrap()).unwrap();

    let mut order: PerpetualOrder = PERPETUAL_ORDER.load(deps.storage, order_id)?;

    let key = order.gen_key()?;
    let mut vec: Vec<u64> = SORTED_PENDING_PERPETUAL_ORDER.load(deps.storage, key.as_str())?;
    let index = vec
        .iter()
        .position(|id| id == &order.order_id)
        .ok_or_else(|| StdError::not_found("order id not found"))?;
    vec.remove(index);

    let res: PerpetualCloseResponse = match get_response_from_reply(module_resp) {
        Ok(expr) => expr,
        Err(err) => {
            order.status = Status::Canceled;
            PERPETUAL_ORDER.save(deps.storage, order_id, &order)?;
            PENDING_PERPETUAL_ORDER.remove(deps.storage, order.order_id);
            return Ok(err);
        }
    };

    order.status = Status::Executed;

    PENDING_PERPETUAL_ORDER.remove(deps.storage, order.order_id);
    PERPETUAL_ORDER.save(deps.storage, order_id, &order)?;

    let resp: Response<ElysMsg> = Response::new().add_event(
        Event::new("reply_to_close_perpetual_order")
            .add_attribute("perpetual_order_id", order_id.to_string())
            .add_attribute("perpetual_trading_position_closed_id", res.id.to_string())
            .add_attribute("perpetual_amount_closed", res.amount.i128().to_string()),
    );

    Ok(resp)
}
