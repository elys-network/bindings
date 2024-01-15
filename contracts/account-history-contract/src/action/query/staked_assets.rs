use super::*;
use crate::{
    msg::query_resp::StakedAssetsResponse,
    types::AccountSnapshot,
};

pub fn staked_assets(deps: Deps<ElysQuery>, address: String)
 -> Result<StakedAssetsResponse, ContractError> {
    let user_history: Vec<crate::types::AccountSnapshot> = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(history) => history,
        None => return Err(StdError::not_found(format!("user :{user_address}"))),
    };

    let latest_snapshot = user_history.last().unwrap();
    Ok(StakedAssetsResponse {
        total_staked_balance: latest_snapshot.total_staked_balance.to_owned(),
        staked_assets: latest_snapshot.staked_assets.to_owned(),
    })
}