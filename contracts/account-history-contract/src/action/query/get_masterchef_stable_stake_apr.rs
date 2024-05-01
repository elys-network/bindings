use cosmwasm_std::{Deps, StdResult};
use elys_bindings::{query_resp::QueryStableStakeAprResponse, ElysQuerier, ElysQuery};

pub fn get_masterchef_stable_stake_apr(
    deps: Deps<ElysQuery>,
    denom: String,
) -> StdResult<QueryStableStakeAprResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp = querier.get_masterchef_stable_stake_apr(denom)?;

    Ok(resp)
}
