use crate::msg::query_resp::StakedAssetsResponse;
use crate::types::AccountSnapshotGenerator;
use cosmwasm_std::{DecCoin, Decimal256, Deps, Env, StdResult};
use elys_bindings::ElysQuery;

pub fn get_staked_assets(
    deps: Deps<ElysQuery>,
    address: String,
    _env: Env,
) -> StdResult<StakedAssetsResponse> {
    let generator = AccountSnapshotGenerator::new(&deps)?;

    let staked_assets_response = generator.get_staked_assets(&deps, &address)?;

    Ok(StakedAssetsResponse {
        total_staked_balance: DecCoin::new(
            Decimal256::from(staked_assets_response.total_staked_balance.amount),
            generator.metadata.usdc_denom,
        ),
        staked_assets: staked_assets_response.staked_assets.to_owned(),
        total_balance: staked_assets_response.total_balance,
        balance_break_down: staked_assets_response.balance_break_down
    })
}
