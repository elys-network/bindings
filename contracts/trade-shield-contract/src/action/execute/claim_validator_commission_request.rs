use cosmwasm_std::StdError;

use super::*;
// delegator_address, validator_address, denom
pub fn claim_validator_commission_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    validator_address: String,
) -> Result<Response<ElysMsg>, ContractError> {
    if REWARD_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("reward endpoint are disable").into());
    }

    let msg = ElysMsg::withdraw_validator_commissions(info.sender.into_string(), validator_address);

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
