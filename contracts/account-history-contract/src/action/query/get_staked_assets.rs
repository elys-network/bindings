use crate::states::HISTORY;
use crate::types::AccountSnapshot;
use crate::{action::VALUE_DENOM, msg::query_resp::StakedAssetsResponse};
use cosmwasm_std::{DecCoin, Decimal256, Deps, StdResult};
use elys_bindings::ElysQuery;

pub fn get_staked_assets(
    deps: Deps<ElysQuery>,
    address: String,
) -> StdResult<StakedAssetsResponse> {
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let snapshots: Vec<crate::types::AccountSnapshot> =
        match HISTORY.may_load(deps.storage, &address)? {
            Some(snapshots) => snapshots,
            None => {
                return Ok(StakedAssetsResponse {
                    total_staked_balance: DecCoin::new(Decimal256::zero(), value_denom.clone()),
                    staked_assets: AccountSnapshot::zero(&value_denom).staked_assets,
                });
            }
        };
    let snapshot = match snapshots.last().cloned() {
        Some(expr) => expr,
        None => {
            return Ok(StakedAssetsResponse {
                total_staked_balance: DecCoin::new(Decimal256::zero(), value_denom.clone()),
                staked_assets: AccountSnapshot::zero(&value_denom).staked_assets,
            });
        }
    };
    Ok(StakedAssetsResponse {
        total_staked_balance: snapshot.portfolio.staked_committed_usd.to_owned(),
        staked_assets: snapshot.staked_assets.to_owned(),
    })
}
