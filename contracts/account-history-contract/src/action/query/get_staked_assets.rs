use crate::states::HISTORY;
use crate::types::AccountSnapshot;
use crate::{action::VALUE_DENOM, msg::query_resp::StakedAssetsResponse};
use cosmwasm_std::{DecCoin, Decimal256, Deps, StdResult};
use elys_bindings::ElysQuery;

pub fn get_staked_assets(
    deps: Deps<ElysQuery>,
    address: String,
) -> StdResult<StakedAssetsResponse> {
    let user_history: Vec<crate::types::AccountSnapshot> =
        match HISTORY.may_load(deps.storage, &address)? {
            Some(history) => history,
            None => {
                let value_denom = VALUE_DENOM.load(deps.storage)?;
                return Ok(StakedAssetsResponse {
                    total_staked_balance: DecCoin::new(Decimal256::zero(), value_denom.clone()),
                    staked_assets: AccountSnapshot::zero(&value_denom).staked_assets,
                });
            }
        };

    let latest_snapshot = user_history.last().unwrap();
    Ok(StakedAssetsResponse {
        total_staked_balance: latest_snapshot.portfolio.staked_committed_usd.to_owned(),
        staked_assets: latest_snapshot.staked_assets.to_owned(),
    })
}
