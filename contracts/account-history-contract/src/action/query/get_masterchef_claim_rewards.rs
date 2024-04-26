use cosmwasm_std::{Deps, StdResult};
use elys_bindings::account_history::msg::query_resp::masterchef::GetMasterchefClaimRewardsResponse;
use elys_bindings::{ElysQuerier, ElysQuery};

pub fn get_masterchef_claim_rewards(
    deps: Deps<ElysQuery>,
    sender: String,
    pool_ids: Vec<u64>,
) -> StdResult<GetMasterchefClaimRewardsResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp = querier.get_masterchef_rewards(sender, pool_ids)?;

    Ok(resp)
}
