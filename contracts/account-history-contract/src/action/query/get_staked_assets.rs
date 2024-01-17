use elys_bindings::ElysQuery;
use crate::msg::query_resp::StakedAssetsResponse;
use cosmwasm_std::{Deps, Decimal, StdResult};
use crate::states::HISTORY;

pub fn get_staked_assets(deps: Deps<ElysQuery>, address: String)
 -> StdResult<StakedAssetsResponse> {
    let user_history: Vec<crate::types::AccountSnapshot> = match HISTORY.may_load(deps.storage, &address)? {
        Some(history) => history,
        None => {
            return Ok(StakedAssetsResponse {
                total_staked_balance: Decimal::zero(),
                staked_assets: vec![],
            })
        }
    };

    let latest_snapshot = user_history.last().unwrap();
    Ok(StakedAssetsResponse {
        total_staked_balance: latest_snapshot.total_staked_asset_balance.to_owned(),
        staked_assets: latest_snapshot.staked_assets.to_owned(),
    })
}