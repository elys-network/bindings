use cosmwasm_std::{Decimal, Deps, StdError, StdResult, Uint128};
use elys_bindings::query_resp::{PoolFilterType, QueryExitPoolEstimationResponse};
use elys_bindings::{ElysQuerier, ElysQuery};

/**
 * Given a pool id and a fiat amount, determine the assets the user will
 * get in return.
 */
pub fn exit_pool_estimation(
    deps: Deps<ElysQuery>,
    pool_id: u64,
    exit_fiat_amount: Decimal,
) -> StdResult<QueryExitPoolEstimationResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    let pool_response =
        querier.get_all_pools(Some(vec![pool_id]), PoolFilterType::FilterAll as i32, None)?;
    let pool = pool_response
        .pools
        .ok_or(StdError::generic_err("Failed to fetch pool"))?
        .first()
        .ok_or(StdError::generic_err("Pool not found"))?
        .clone();
    let share_price = pool
        .share_usd_price
        .ok_or(StdError::generic_err("Unable to get share price"))?;
    let share_amount = exit_fiat_amount
        .checked_div(share_price)
        .unwrap_or_default();

    Ok(querier
        .exit_pool_estimation(pool_id, Uint128::new(share_amount.atomics().into()))
        .map_err(|err| {
            StdError::generic_err(format!(
                "exit_pool_estimation: Error getting estimation from chain:{:?}",
                err
            ))
        })?)
}
