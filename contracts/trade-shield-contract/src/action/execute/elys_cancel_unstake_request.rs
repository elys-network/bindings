use super::*;
use cosmwasm_std::{Coin, StdError};

pub fn elys_cancel_unstake_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    validator_address: String,
    amount: Coin,
    creation_height: i64,
) -> Result<Response<ElysMsg>, ContractError> {
    if STAKE_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("stake endpoint are disable").into());
    }
    if amount.amount.is_zero() {
        return Err(StdError::generic_err("amount is zero").into());
    }

    let msg = ElysMsg::cancel_unbonding(
        info.sender.into_string(),
        validator_address,
        amount,
        creation_height,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
