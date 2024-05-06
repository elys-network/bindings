use cosmwasm_std::{Deps, StdResult};
use elys_bindings::{
    account_history::msg::query_resp::masterchef::MasterChefPoolAprResponse, ElysQuerier, ElysQuery,
};

pub fn get_masterchef_pool_apr(
    deps: Deps<ElysQuery>,
    pool_ids: Vec<u64>,
) -> StdResult<MasterChefPoolAprResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp = querier.get_masterchef_pool_apr(pool_ids)?;

    Ok(MasterChefPoolAprResponse {
        data: resp.to_decimal(),
    })
}
