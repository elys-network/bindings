use cosmwasm_std::{Deps, StdResult};
use elys_bindings::account_history::msg::query_resp::estaking::GetEstakingRewardsResponse;
use elys_bindings::account_history::types::CoinValue;
use elys_bindings::{ElysQuerier, ElysQuery};

/**
 * Given a user address, gets the Estaking rewards available.
 */
pub fn get_estaking_rewards(
    deps: Deps<ElysQuery>,
    address: String,
) -> StdResult<GetEstakingRewardsResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    let response = querier.get_estaking_rewards(address).unwrap_or_default();

    let fiat_coins = response.to_coin_values(&querier);
    let rewards = fiat_coins
        .unwrap_or_default()
        .into_iter()
        .map(|(_, v)| v.clone())
        .collect::<Vec<CoinValue>>();

    Ok(GetEstakingRewardsResponse {
        rewards,
        total: response.total,
    })
}
