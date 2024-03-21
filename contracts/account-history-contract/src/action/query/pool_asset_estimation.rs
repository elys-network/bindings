use std::collections::HashMap;
use cosmwasm_std::{Deps, StdResult, StdError, Decimal, DecCoin, Decimal256};
use elys_bindings::{ElysQuerier, ElysQuery};
use elys_bindings::query_resp::{PoolFilterType, QueryPoolAssetEstimationResponse};

/**
 * Given an asset and a pool, determine the quantity of every other asset in the pool
 * needed to keep the pool balanced.
 * Useful to use in FE forms before calling join pool.
 */
pub fn pool_asset_estimation(
    deps: Deps<ElysQuery>,
    pool_id: u64,
    asset: DecCoin
) -> StdResult<QueryPoolAssetEstimationResponse> {
    let querier = ElysQuerier::new(&deps.querier);
    let asset_denom = asset.denom.to_string();

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

    let asset_usd_price = querier.get_asset_price(asset.denom).unwrap_or(Decimal::zero());
    let asset_in_usd = asset.amount * Decimal256::from(asset_usd_price);

    // Ensure the current_pool_ratio is populated
    let current_pool_ratio = match &pool.current_pool_ratio {
        Some(ratio) => ratio,
        None => return Err(StdError::generic_err("Current pool ratio is not populated")),
    };

    let asset_ratio = Decimal256::from(current_pool_ratio.get(&asset_denom).unwrap().clone());
    let total_in_usd = asset_in_usd / asset_ratio;
    
    let mut estimations = HashMap::new();
    for (denom, _) in current_pool_ratio.iter() {
        if denom.to_string() != asset_denom {
            let usd_price = querier.get_asset_price(denom).unwrap_or(Decimal::zero());
            let dec_price = Decimal256::from(usd_price);
            let ratio = Decimal256::from(current_pool_ratio.get(denom).unwrap().clone());

            let quantity = (total_in_usd * ratio) / dec_price;

            estimations.insert(denom.clone(), quantity);
        }
    }

    // Return the result
    Ok(QueryPoolAssetEstimationResponse { amounts: estimations })
}
