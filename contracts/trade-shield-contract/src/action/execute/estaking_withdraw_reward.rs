use cosmwasm_std::StdError;

use super::*;

/**
 * Given a validator address, it will claim the user (info.sender) rewards for the given validator.
 * To claim Eden rewards, the validator would be: elysvaloper1gnmpr8vvslp3shcq6e922xr0uq4aa2w5gdzht0
 * To claim Eden boost rewards, the validator would be: elysvaloper1wajd6ekh9u37hyghyw4mme59qmjllzuyaceanm
 */
pub fn estaking_withdraw_reward(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    validator_address: String,
) -> Result<Response<ElysMsg>, ContractError> {
    if REWARD_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("reward endpoint are disabled").into());
    }

    let msg = ElysMsg::estaking_withdraw_reward(info.sender.into_string(), validator_address);

    let response = Response::new().add_message(msg);

    Ok(response)
}
