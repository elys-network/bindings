use super::*;
use cosmwasm_std::Int128;

pub fn close_leveragelp_position_request(
    info: MessageInfo,
    position_id: u64,
    amount: Int128,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg: ElysMsg =
        ElysMsg::leveragelp_close_position(info.sender.into_string(), position_id, amount);

    let resp = Response::new().add_message(msg);
    Ok(resp)
}
