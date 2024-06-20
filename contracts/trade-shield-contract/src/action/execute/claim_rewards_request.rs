use super::*;
use cosmwasm_std::StdError;
use elys_bindings::query_resp::Validator;

pub fn claim_rewards_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
) -> Result<Response<ElysMsg>, ContractError> {
    if REWARD_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("reward endpoint are disable").into());
    }

    let mut msgs: Vec<ElysMsg> = vec![];
    let querier = ElysQuerier::new(&deps.querier);

    // estaking withdraw elys staking rewards
    msgs.push(ElysMsg::estaking_withdraw_elys_staking_rewards(
        info.sender.to_string(),
    ));

    // estaking withdraw reward
    let estaking_reward: query_resp::EstakingRewardsResponse =
        querier.get_estaking_rewards(info.sender.to_string())?;

    if estaking_reward
        .get_validator_rewards(Validator::Eden)
        .rewards[0]
        .reward
        .is_empty()
        == false
    {
        msgs.push(ElysMsg::estaking_withdraw_reward(
            info.sender.to_string(),
            Validator::Eden.to_string(),
        ));
    }

    if estaking_reward
        .get_validator_rewards(Validator::EdenBoost)
        .rewards[0]
        .reward
        .is_empty()
        == false
    {
        msgs.push(ElysMsg::estaking_withdraw_reward(
            info.sender.to_string(),
            Validator::EdenBoost.to_string(),
        ));
    }

    let master_chef_pending_rewards =
        querier.get_masterchef_pending_rewards(info.sender.to_string())?;

    if master_chef_pending_rewards.total_rewards.is_empty() == false {
        let pools_ids_to_claim: Vec<u64> = master_chef_pending_rewards
            .rewards
            .iter()
            .filter_map(|reward| {
                if reward.reward.is_empty() {
                    None
                } else {
                    Some(reward.pool_id)
                }
            })
            .collect();

        msgs.push(ElysMsg::get_masterchef_claim_rewards(
            info.sender.to_string(),
            pools_ids_to_claim,
        ));
    }
    let ids = querier.leveragelp_pool_position_ids_for_address(info.sender.to_string())?;

    if !ids.is_empty() {
        msgs.push(ElysMsg::leveragelp_withdraw_reward(
            info.sender.to_string(),
            ids,
        ));
    }

    let resp = Response::new().add_messages(msgs);

    Ok(resp)
}
