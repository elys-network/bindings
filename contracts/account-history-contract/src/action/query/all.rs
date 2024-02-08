use cosmwasm_std::{Deps, Order, StdResult};
use elys_bindings::ElysQuery;

use crate::{states::HISTORY, types::AccountSnapshot};

pub fn all(deps: Deps<ElysQuery>) -> StdResult<Vec<(String, Vec<(String, AccountSnapshot)>)>> {
    let list: Vec<(String, Vec<(String, AccountSnapshot)>)> = HISTORY
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok())
        .map(|(key, value)| {
            let account_snapshots: Vec<(String, AccountSnapshot)> = value.into_iter().collect();

            (key, account_snapshots)
        })
        .collect();

    Ok(list)
}
