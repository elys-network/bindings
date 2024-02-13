use cosmwasm_std::{Deps, Env, StdResult};
use elys_bindings::{account_history::types::PerpetualAssets, ElysQuery};

use crate::types::AccountSnapshotGenerator;

pub fn get_perpetuals_assets(
    deps: Deps<ElysQuery>,
    address: String,
    _env: Env,
) -> StdResult<PerpetualAssets> {
    let generator = AccountSnapshotGenerator::new(&deps)?;

    generator.get_perpetuals(&deps, &address)
}
