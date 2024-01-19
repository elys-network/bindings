use crate::{
    action::{HISTORY, VALUE_DENOM},
    msg::query_resp::GetRewardsResp,
    types::AccountSnapshot,
};

use cosmwasm_std::{Deps, StdResult};

use elys_bindings::ElysQuery;

pub fn get_rewards(deps: Deps<ElysQuery>, user_address: String) -> StdResult<GetRewardsResp> {
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let snapshots = HISTORY.load(deps.storage, &user_address)?;
    let snapshot = match snapshots.last().cloned() {
        Some(expr) => expr,
        None => {
            return Ok(GetRewardsResp {
                rewards: AccountSnapshot::zero(&value_denom).reward,
            })
        }
    };
    let resp = GetRewardsResp {
        rewards: snapshot.reward,
    };
    Ok(resp)
}
