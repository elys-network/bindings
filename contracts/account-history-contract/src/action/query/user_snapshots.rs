use std::cmp::Ordering;

use cosmwasm_std::{Deps, StdResult};
use elys_bindings::ElysQuery;

use crate::{action::HISTORY, types::AccountSnapshot};

pub fn user_snapshots(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<Vec<AccountSnapshot>> {
    let mut snapshots: Vec<AccountSnapshot> = HISTORY
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
