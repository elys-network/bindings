use cosmwasm_std::{MessageInfo, Response};
use elys_bindings::ElysMsg;

use crate::ContractError;

pub fn masterchef_claim_rewards(
    info: MessageInfo,
    pool_ids: Vec<u64>,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg: ElysMsg = ElysMsg::get_masterchef_claim_rewards(info.sender.into_string(), pool_ids);
    Ok(Response::default().add_message(msg))
}
