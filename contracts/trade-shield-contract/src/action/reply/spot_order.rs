use cosmwasm_std::{from_json, Binary, DepsMut, SubMsgResult};

use crate::helper::get_response_from_reply;

use super::*;

pub fn reply_to_spot_order(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let order_id: u64 = from_json(&data.unwrap()).unwrap();

    let mut order = SPOT_ORDER.load(deps.storage, order_id)?;

    let key = order.gen_key()?;
    let mut vec: Vec<u64> = SORTED_PENDING_SPOT_ORDER.load(deps.storage, key.as_str())?;
    let mut index = SpotOrder::binary_search(&order.order_price.rate, deps.storage, &vec)?;
    let size_of_vec = vec.len();
    while vec[index] != order_id && index < size_of_vec {
        index += 1;
    }
    if index < size_of_vec {
        vec.remove(index);
    }

    SORTED_PENDING_SPOT_ORDER.save(deps.storage, key.as_str(), &vec)?;

    let _: AmmSwapExactAmountInResp = match get_response_from_reply(module_resp) {
        Ok(expr) => expr,
        Err(err) => {
            order.status = Status::Canceled;
            SPOT_ORDER.save(deps.storage, order_id, &order)?;
            PENDING_SPOT_ORDER.remove(deps.storage, order.order_id);
            return Ok(err.add_message(BankMsg::Send {
                to_address: order.owner_address.to_string(),
                amount: vec![order.order_amount],
            }));
        }
    };

    order.status = Status::Executed;

    SPOT_ORDER.save(deps.storage, order_id, &order)?;
    PENDING_SPOT_ORDER.remove(deps.storage, order.order_id);

    let resp: Response<ElysMsg> = Response::new().add_event(
        Event::new("reply_to_spot_order").add_attribute("order_id", order_id.to_string()),
    );

    Ok(resp)
}
