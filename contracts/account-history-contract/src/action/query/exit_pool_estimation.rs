use cosmwasm_std::{Decimal, Deps, StdError, StdResult, Uint128};
use elys_bindings::{ElysQuerier, ElysQuery};
use elys_bindings::query_resp::{PoolFilterType, QueryExitPoolEstimationResponse};

/**
 * Given a pool id and a fiat amount, determine the assets the user will
 * get in return.
 */
pub fn exit_pool_estimation(
    deps: Deps<ElysQuery>,
    pool_id: u64,
    exit_fiat_amount: Decimal
) -> StdResult<QueryExitPoolEstimationResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    let pool_response = querier.get_all_pools(Some(vec![pool_id]), PoolFilterType::FilterAll as i32, None)?;
    let pool = match pool_response.pools {
        Some(pools) => {
            if let Some(pool) = pools.first() {
                pool.clone()
            } else {
                return Err(StdError::generic_err("Pool not found"));
            }
        }
        None => return Err(StdError::generic_err("Failed to fetch pool")),
    };

    let share_price = match pool.share_usd_price {
        Some(share) => share,
        None => return Err(StdError::generic_err("Unable to get share price"))
    };
    let share_amount = exit_fiat_amount
        .checked_div(share_price)
        .unwrap_or_default();

    match querier.exit_pool_estimation(pool_id, Uint128::new(share_amount.atomics().into())) {
        Ok(response) => Ok(response),
        Err(err) => return Err(StdError::generic_err(format!("exit_pool_estimation: Error getting estimation from chain:{:?}", err)))
    }
}
