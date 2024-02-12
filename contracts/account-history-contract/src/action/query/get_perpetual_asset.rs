use cosmwasm_std::{Deps, Env, StdResult};
use elys_bindings::{account_history::types::PerpetualAssets, ElysQuerier, ElysQuery};

use crate::{
    states::{EXPIRATION, TRADE_SHIELD_ADDRESS},
    types::AccountSnapshotGenerator,
};

pub fn get_perpetuals_assets(
    deps: Deps<ElysQuery>,
    address: String,
    env: Env,
) -> StdResult<PerpetualAssets> {
    let querier = ElysQuerier::new(&deps.querier);

    let expiration = EXPIRATION.load(deps.storage)?;
    let trade_shield_address = TRADE_SHIELD_ADDRESS.load(deps.storage)?;

    let generator = AccountSnapshotGenerator::new(&querier, trade_shield_address, expiration)?;

    // let snapshot =
    //     match generator.generate_account_snapshot_for_address(&querier, &deps, &env, &address)? {
    //         Some(snapshot) => snapshot,
    //         None => {
    //             return Ok(PerpetualAssets::default());
    //         }
    //     };

    generator.get_perpetuals(&deps, address)

    // Ok(snapshot.perpetual_assets)
}
