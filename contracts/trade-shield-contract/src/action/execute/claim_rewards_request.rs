use super::*;
use cosmwasm_std::StdError;
use elys_bindings::types::EarnType;

pub fn claim_rewards_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    withdraw_type: EarnType,
) -> Result<Response<ElysMsg>, ContractError> {
    if REWARD_ENABLE.load(deps.storage)? == false {
        return Err(StdError::generic_err("reward endpoint are disable").into());
    }

    let msg = ElysMsg::withdraw_rewards(info.sender.into_string(), withdraw_type);

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
