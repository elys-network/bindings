use super::*;
use crate::{bindings::{querier::ElysQuerier, query::ElysQuery}, msg::query_resp::earn::{FilterType, QueryEarnPoolResponse}};
use crate::types::PageRequest;

pub fn get_pools(deps: Deps<ElysQuery>, pool_ids: Option<Vec<u64>>, filter_type: FilterType, pagination: Option<PageRequest>) -> Result<QueryEarnPoolResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);
    let resp = querier.get_all_pools(pool_ids, filter_type as i32, pagination)?;
    Ok(resp)
}