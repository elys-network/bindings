use cosmwasm_std::{Deps, Env, StdResult};
use elys_bindings::ElysQuery;

use crate::{states::HISTORY, types::PerpetualAssets, utils::get_today};

pub fn get_perpetuals_assets(
    deps: Deps<ElysQuery>,
    address: String,
    env: Env,
) -> StdResult<PerpetualAssets> {
    let snapshots = match HISTORY.may_load(deps.storage, &address)? {
        Some(snapshots) => snapshots,
        None => return Ok(PerpetualAssets::default()),
    };

    let today = get_today(&env.block);

    let perpetual_assets = match snapshots.get(&today) {
        Some(snapshot) => snapshot.perpetual_assets.clone(),
        None => PerpetualAssets::default(),
    };

    Ok(perpetual_assets)
}
