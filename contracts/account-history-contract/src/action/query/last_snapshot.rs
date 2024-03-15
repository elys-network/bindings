use cosmwasm_std::{Deps, Env, StdResult};
use elys_bindings::{account_history::types::PortfolioBalanceSnapshot, ElysQuery};

use crate::{states::HISTORY, utils::get_today};

pub fn last_snapshot(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
) -> StdResult<PortfolioBalanceSnapshot> {
    let today = get_today(&env.block);

    let snapshot = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => match snapshots.get(&today) {
            Some(snapshot) => snapshot.clone(),
            None => PortfolioBalanceSnapshot::default(),
        },
        None => PortfolioBalanceSnapshot::default(),
    };

    Ok(snapshot)
}
