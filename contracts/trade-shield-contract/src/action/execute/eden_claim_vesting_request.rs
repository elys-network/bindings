use super::*;

pub fn eden_claim_vesting_request(info: MessageInfo) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::eden_claim_vesting(info.sender.into_string());

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
