use crate::states::HISTORY;
use crate::{msg::query_resp::StakedAssetsResponse, utils::get_today};
use cosmwasm_std::{DecCoin, Decimal256, Deps, Env, StdResult};
use elys_bindings::account_history::types::AccountSnapshot;
use elys_bindings::{
    query_resp::{Entry, QueryGetEntryResponse},
    ElysQuerier, ElysQuery,
};

pub fn get_staked_assets(
    deps: Deps<ElysQuery>,
    address: String,
    env: Env,
) -> StdResult<StakedAssetsResponse> {
    let querier = ElysQuerier::new(&deps.querier);
    let QueryGetEntryResponse {
        entry: Entry {
            denom: usdc_denom, ..
        },
    } = querier.get_asset_profile("uusdc".to_string())?;
    let snapshots = match HISTORY.may_load(deps.storage, &address)? {
        Some(snapshots) => snapshots,
        None => {
            return Ok(StakedAssetsResponse {
                total_staked_balance: DecCoin::new(Decimal256::zero(), usdc_denom.clone()),
                staked_assets: AccountSnapshot::zero(&usdc_denom).staked_assets,
            });
        }
    };

    let today = get_today(&env.block);

    let snapshot = match snapshots.get(&today) {
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
