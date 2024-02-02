use crate::{action::HISTORY, msg::query_resp::GetRewardsResp, types::AccountSnapshot};

use cosmwasm_std::{Deps, StdResult};

use elys_bindings::{
    query_resp::{Entry, QueryGetEntryResponse},
    ElysQuerier, ElysQuery,
};

pub fn get_rewards(deps: Deps<ElysQuery>, user_address: String) -> StdResult<GetRewardsResp> {
    let querier = ElysQuerier::new(&deps.querier);
    let QueryGetEntryResponse {
        entry: Entry {
            denom: usdc_denom, ..
        },
    } = querier.get_asset_profile("uusdc".to_string())?;

    let snapshots = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => snapshots,
        None => {
            return Ok(GetRewardsResp {
                rewards: AccountSnapshot::zero(&usdc_denom).reward,
            })
        }
    };
    let snapshot = match snapshots.last().cloned() {
        Some(expr) => expr,
        None => {
            return Ok(GetRewardsResp {
                rewards: AccountSnapshot::zero(&usdc_denom).reward,
            })
        }
    };
    let resp = GetRewardsResp {
        rewards: snapshot.reward,
    };
    Ok(resp)
}
