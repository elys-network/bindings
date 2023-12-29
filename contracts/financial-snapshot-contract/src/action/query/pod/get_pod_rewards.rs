use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::pod::GetRewardsResp};

pub fn get_pod_rewards(deps: Deps<ElysQuery>, address: String) -> Result<GetRewardsResp, ContractError> {
    let ret = REWARDS.may_load(deps.storage, &address);
    let resp = GetRewardsResp {
        rewards: match ret {
            Ok(Some(data)) => data.to_owned(),
            Ok(None) => Reward::new_dummy(),
            Err(_) => return Err(ContractError::RewardError{}),
        },
    };

    Ok(resp)
}
