use cosmwasm_std::{Deps, StdResult};
use elys_bindings::ElysQuery;

use crate::{states::HISTORY, types::PerpetualAssets};

pub fn get_perpetuals_asset(deps: Deps<ElysQuery>, address: String) -> StdResult<PerpetualAssets> {
    let snapshots = HISTORY.may_load(deps.storage, &address)?;

    let perpetual_assets = match snapshots {
        Some(snapshots) => match snapshots.last().cloned() {
            Some(snapshot) => snapshot.perpetual_assets,
            None => PerpetualAssets::default(),
        },
        None => PerpetualAssets::default(),
    };

    Ok(perpetual_assets)
}
