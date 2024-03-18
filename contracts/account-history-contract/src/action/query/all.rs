use cosmwasm_std::{Deps, Order, StdResult};
use elys_bindings::{
    account_history::{msg::query_resp::GetAllResp, types::PortfolioBalanceSnapshot},
    types::PageRequest,
    ElysQuery,
};

use crate::states::HISTORY;

pub fn all(deps: Deps<ElysQuery>, pagination: Option<PageRequest>) -> StdResult<GetAllResp> {
    let snapshot_list: Vec<(String, Vec<(String, PortfolioBalanceSnapshot)>)> = HISTORY
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok())
        .map(|(key, value)| {
            let account_snapshots: Vec<(String, PortfolioBalanceSnapshot)> =
                value.into_iter().collect();

            (key, account_snapshots)
        })
        .collect();

    let (snapshot_list, page_response) = match pagination {
        Some(pagination) => {
            let (snapshot_list, page_resp) = pagination.filter(snapshot_list)?;
            (snapshot_list, Some(page_resp))
        }
        None => (snapshot_list, None),
    };

    Ok(GetAllResp {
        snapshot_list,
        pagination: page_response,
    })
}
