use super::*;
use cosmwasm_std::StdError;

pub fn estaking_withdraw_elys_staking_rewards(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
) -> Result<Response<ElysMsg>, ContractError> {
    if REWARD_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("reward endpoint are disable").into());
    }

    let message = ElysMsg::estaking_withdraw_elys_staking_rewards(info.sender.into_string());

    let response = Response::new().add_message(message);

    Ok(response)
}
