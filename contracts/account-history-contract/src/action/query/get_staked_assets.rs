use crate::msg::query_resp::StakedAssetsResponse;
use crate::states::HISTORY;
use crate::types::AccountSnapshot;
use cosmwasm_std::{DecCoin, Decimal256, Deps, StdResult};
use elys_bindings::{
    query_resp::{Entry, QueryGetEntryResponse},
    ElysQuerier, ElysQuery,
};

pub fn get_staked_assets(
    deps: Deps<ElysQuery>,
    address: String,
) -> StdResult<StakedAssetsResponse> {
    let querier = ElysQuerier::new(&deps.querier);
    let QueryGetEntryResponse {
        entry: Entry {
            denom: usdc_denom, ..
        },
    } = querier.get_asset_profile("uusdc".to_string())?;
    let snapshots: Vec<crate::types::AccountSnapshot> =
        match HISTORY.may_load(deps.storage, &address)? {
            Some(snapshots) => snapshots,
            None => {
                return Ok(StakedAssetsResponse {
                    total_staked_balance: DecCoin::new(Decimal256::zero(), usdc_denom.clone()),
                    staked_assets: AccountSnapshot::zero(&usdc_denom).staked_assets,
                });
            }
        };
    let snapshot = match snapshots.last().cloned() {
        Some(expr) => expr,
        None => {
            return Ok(StakedAssetsResponse {
                total_staked_balance: DecCoin::new(Decimal256::zero(), usdc_denom.clone()),
                staked_assets: AccountSnapshot::zero(&usdc_denom).staked_assets,
            });
        }
    };
    Ok(StakedAssetsResponse {
        total_staked_balance: snapshot.portfolio.staked_committed_usd.to_owned(),
        staked_assets: snapshot.staked_assets.to_owned(),
    })
}
