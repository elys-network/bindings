use super::*;
use cosmwasm_std::{Int128, StdError};

pub fn close_leveragelp_position_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    position_id: u64,
    amount: Int128,
) -> Result<Response<ElysMsg>, ContractError> {
    if LEVERAGE_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("leverage endpoint are disable").into());
    }

    let msg: ElysMsg =
        ElysMsg::leveragelp_close_position(info.sender.into_string(), position_id, amount);

    let resp = Response::new().add_message(msg);
    Ok(resp)
}
