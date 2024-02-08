use crate::{
    msg::query_resp::GetRewardsResp, states::HISTORY, types::AccountSnapshot, utils::get_today,
};

use cosmwasm_std::{Deps, Env, StdResult};

use elys_bindings::{
    query_resp::{Entry, QueryGetEntryResponse},
    ElysQuerier, ElysQuery,
};

pub fn get_rewards(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
) -> StdResult<GetRewardsResp> {
    let querier = ElysQuerier::new(&deps.querier);
    let QueryGetEntryResponse {
        entry: Entry {
            denom: usdc_denom, ..
        },
    } = querier.get_asset_profile("uusdc".to_string())?;

    let snapshots = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => snapshots,
        None => {
            return Ok(GetRewardsResp {
                rewards: AccountSnapshot::zero(&usdc_denom).reward,
            })
        }
    };

    let today = get_today(&env.block);

    let snapshot = match snapshots.get(&today) {
        Some(expr) => expr,
        None => {
            return Ok(GetRewardsResp {
                rewards: AccountSnapshot::zero(&usdc_denom).reward,
            })
        }
    };

    let resp = GetRewardsResp {
        rewards: snapshot.reward.clone(),
    };

    Ok(resp)
}
