use cosmwasm_std::{Deps, Env, StdResult};
use elys_bindings::ElysQuery;

use crate::{action::HISTORY, types::AccountSnapshot, utils::get_today};

pub fn last_snapshot(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
) -> StdResult<AccountSnapshot> {
    let today = get_today(&env.block);

    let snapshot = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => match snapshots.get(&today) {
            Some(snapshot) => snapshot.clone(),
            None => AccountSnapshot::default(),
        },
        None => AccountSnapshot::default(),
    };

    Ok(snapshot)
}
