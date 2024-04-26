use cosmwasm_std::{Deps, StdResult};
use elys_bindings::account_history::msg::query_resp::masterchef::GetMasterchefUserPendingRewardResponse;
use elys_bindings::{ElysQuerier, ElysQuery};

use crate::types::AccountSnapshotGenerator;

pub fn get_masterchef_pending_rewards(
    deps: Deps<ElysQuery>,
    address: String,
) -> StdResult<GetMasterchefUserPendingRewardResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    let generator = AccountSnapshotGenerator::new(&deps)?;

    let resp = querier.get_masterchef_pending_rewards(address)?;

    let (rewards, total_rewards) =
        resp.to_dec_coin_values(&querier, &generator.metadata.usdc_denom)?;

    Ok(GetMasterchefUserPendingRewardResponse {
        rewards,
        total_rewards,
    })
}
