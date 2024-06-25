use crate::{msg::query_resp::GetRewardsResp, types::AccountSnapshotGenerator};

use cosmwasm_std::{Deps, Env, StdResult, Storage};

use elys_bindings::ElysQuery;

pub fn get_rewards(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
    storage: &mut dyn Storage,
) -> StdResult<GetRewardsResp> {
    let generator = AccountSnapshotGenerator::new(&deps)?;

    let rewards_response = generator.get_rewards(&deps, &user_address, env, storage)?;

    let resp = GetRewardsResp {
        rewards_map: rewards_response.rewards_map.clone(),
        rewards: rewards_response.rewards.clone(),
    };

    Ok(resp)
}
