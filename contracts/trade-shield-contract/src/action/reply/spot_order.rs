use cosmwasm_std::{from_json, Binary, DepsMut, SubMsgResult};

use crate::helper::{get_response_from_reply, remove_spot_order};

use super::*;

pub fn reply_to_spot_order(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let order_id: u64 = from_json(&data.unwrap()).unwrap();

    let _: AmmSwapExactAmountInResp = match get_response_from_reply(module_resp) {
        Ok(expr) => expr,
        Err(err) => {
            let bank_msg = remove_spot_order(order_id, Status::Canceled, deps.storage)?;
            return Ok(err.add_message(bank_msg.unwrap()));
        }
    };

    remove_spot_order(order_id, Status::Executed, deps.storage)?;

    let resp: Response<ElysMsg> = Response::new().add_event(
        Event::new("reply_to_spot_order").add_attribute("order_id", order_id.to_string()),
    );

    Ok(resp)
}
