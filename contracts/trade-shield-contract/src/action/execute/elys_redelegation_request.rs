use super::*;
use cosmwasm_std::{Coin, StdError};

pub fn elys_redelegation_request(
    info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    // the amount to be staked in base denomination.
    validator_src_address: String,
    // The asset to be staked
    validator_dst_address: String,
    // The validator Address is required only if the staked asset is
    // uelys.
    amount: Coin,
) -> Result<Response<ElysMsg>, ContractError> {
    let uelys_denom = "uelys".to_string();

    if amount.amount.is_zero() {
        return Err(StdError::generic_err("amount is zero").into());
    }

    if (validator_dst_address.is_empty() || validator_src_address.is_empty())
        && amount.denom == uelys_denom
    {
        return Err(StdError::generic_err(
            "The validator Address is required only if the staked asset is uelys",
        )
        .into());
    };

    let msg = ElysMsg::begin_redelegate(
        info.sender.into_string(),
        validator_src_address,
        validator_dst_address,
        amount,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
