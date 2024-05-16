use cosmwasm_std::{Deps, StdError, StdResult};
use elys_bindings::account_history::msg::query_resp::estaking::GetEstakingRewardsResponse;
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

    let fiat_coins = response.to_coin256_values(&querier);

    match fiat_coins {
        Err(e) => {
            return Err(StdError::generic_err(format!(
                "Failed to convert to Coin256Value {}",
                e
            )))
        }
        _ => {}
    }

    Ok(GetEstakingRewardsResponse {
        rewards: fiat_coins.unwrap_or_default(),
        total: response.total,
    })
}
