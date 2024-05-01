use cosmwasm_std::{from_json, Binary, SubMsgResult};

use crate::helper::{get_response_from_reply, remove_perpetual_order};

use super::*;

pub fn reply_to_open_perpetual_position(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let order_id: u64 = from_json(&data.unwrap()).unwrap();

    let res: PerpetualOpenResponse = match get_response_from_reply(module_resp) {
        Ok(expr) => expr,
        Err(err) => {
            let bank_msg =
                remove_perpetual_order(order_id, Status::Canceled, deps.storage, None)?.unwrap();
            return Ok(err.add_message(bank_msg));
        }
    };

    remove_perpetual_order(order_id, Status::Executed, deps.storage, Some(res.id))?;

    let resp: Response<ElysMsg> = Response::new().add_event(
        Event::new("reply_to_open_perpetual_position")
            .add_attribute("perpetual_order_id", order_id.to_string())
            .add_attribute("perpetual_trading_position_opened_id", res.id.to_string()),
    );

    Ok(resp)
}
