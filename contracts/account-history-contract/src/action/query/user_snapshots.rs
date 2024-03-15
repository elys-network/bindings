use std::cmp::Ordering;

use cosmwasm_std::{Deps, StdResult};
use elys_bindings::{account_history::types::PortfolioBalanceSnapshot, ElysQuery};

use crate::states::HISTORY;

pub fn user_snapshots(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<Vec<PortfolioBalanceSnapshot>> {
    let mut snapshots: Vec<PortfolioBalanceSnapshot> = HISTORY
        .load(deps.storage, &user_address)?
        .values()
        .cloned()
        .collect();

    snapshots.sort_by(|a, b| match a.date.partial_cmp(&b.date) {
        Some(order) => order,
        None => Ordering::Less,
    });

    Ok(snapshots)
}
