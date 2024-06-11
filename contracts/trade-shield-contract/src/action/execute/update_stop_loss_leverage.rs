use super::*;
use cosmwasm_std::{SignedDecimal, StdError};

pub fn update_stop_loss_leveragelp_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    position: u64,
    price: SignedDecimal,
) -> Result<Response<ElysMsg>, ContractError> {
    if LEVERAGE_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("leverage endpoint are disable").into());
    }

    let msg: ElysMsg =
        ElysMsg::leveragelp_update_stop_loss(info.sender.to_string(), position, price);

    let resp = Response::new().add_message(msg);
    Ok(resp)
}
