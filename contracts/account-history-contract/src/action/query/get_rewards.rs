use crate::{msg::query_resp::GetRewardsResp, types::AccountSnapshotGenerator};

use cosmwasm_std::{Deps, Env, StdResult};

use elys_bindings::ElysQuery;

pub fn get_rewards(
    deps: Deps<ElysQuery>,
    user_address: String,
    _env: Env,
) -> StdResult<GetRewardsResp> {
    let generator = AccountSnapshotGenerator::new(&deps)?;

    let rewards_response = generator.get_rewards(&deps, &user_address)?;

    let resp = GetRewardsResp {
        rewards: rewards_response.rewards.clone(),
    };

    Ok(resp)
}
