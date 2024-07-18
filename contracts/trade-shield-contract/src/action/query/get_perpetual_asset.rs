use cosmwasm_std::{DecCoin, Decimal256, Deps, Env, StdResult};
use elys_bindings::{
    query_resp::PerpetualGetPositionsForAddressResponse,
    trade_shield::types::PerpetualPositionPlus,
    trade_shield::types::{PerpetualAsset, PerpetualAssets},
    ElysQuerier, ElysQuery,
};

pub fn get_perpetuals_assets(
    deps: Deps<ElysQuery>,
    address: String,
    _env: Env,
    usdc_denom: String,
) -> StdResult<PerpetualAssets> {
    let querier = ElysQuerier::new(&deps.querier);

    let PerpetualGetPositionsForAddressResponse { mtps, .. } =
        querier.perpetual_get_position_for_address(address, None)?;

    let mtps = PerpetualPositionPlus::news(mtps, deps.storage, &querier)?;
    let mut perpetual_vec: Vec<PerpetualAsset> = vec![];

    for mtp in mtps {
        match PerpetualAsset::new(mtp, usdc_denom.to_owned(), &querier) {
            Ok(perpetual_asset) => perpetual_vec.push(perpetual_asset),
            Err(_) => continue,
        }
    }

    let total_perpetual_asset_balance_amount = perpetual_vec
        .iter()
        .map(|perpetual| perpetual.size.amount)
        .fold(Decimal256::zero(), |acc, item| acc + item);
    let total_perpetual_asset_balance =
        DecCoin::new(total_perpetual_asset_balance_amount, usdc_denom.to_owned());

    Ok(PerpetualAssets {
        total_perpetual_asset_balance,
        perpetual_asset: perpetual_vec,
    })
}
