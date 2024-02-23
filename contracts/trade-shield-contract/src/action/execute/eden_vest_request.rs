use super::*;
use cosmwasm_std::{Int128, StdError};

pub fn eden_vest_request(
    info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    amount: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    if amount == 0 {
        return Err(StdError::generic_err("amount is zero").into());
    }
    let msg: ElysMsg = ElysMsg::eden_vesting(
        info.sender.into_string(),
        Int128::from(amount),
        "ueden".to_string(),
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
