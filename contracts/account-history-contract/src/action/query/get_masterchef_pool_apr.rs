use cosmwasm_std::{Deps, StdResult};
use elys_bindings::query_resp::MasterChefPoolAprResponse;
use elys_bindings::{ElysQuerier, ElysQuery};

use crate::types::AccountSnapshotGenerator;

pub fn get_masterchef_pool_apr(
    deps: Deps<ElysQuery>,
    pool_ids: Vec<u64>,
) -> StdResult<MasterChefPoolAprResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    let generator = AccountSnapshotGenerator::new(&deps)?;

    let resp = querier.get_masterchef_pool_apr(pool_ids)?;

    let result = resp
        .data
        .iter()
        .map(|value| {
            value
                .to_dec_coin_value(&querier, &generator.metadata.usdc_denom)
                .unwrap_or_default()
        })
        .collect();

    Ok(MasterChefPoolAprResponse { data: result })
}
