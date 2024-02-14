use cosmwasm_std::{Decimal, Deps, Env, StdResult, Coin};
use elys_bindings::{account_history::types::CoinValue, ElysQuerier, ElysQuery};

use crate::{
    msg::query_resp::{GetLiquidAssetsResp, LiquidAsset},
    types::AccountSnapshotGenerator,
};

pub fn get_liquid_assets(
    deps: Deps<ElysQuery>,
    user_address: String,
    _env: Env,
) -> StdResult<GetLiquidAssetsResp> {
    let querier = ElysQuerier::new(&deps.querier);

    let generator = AccountSnapshotGenerator::new(&deps)?;

    let liquid_asset = generator.get_liquid_assets(&deps, &querier, &user_address)?;

    let mut liquid_assets: Vec<LiquidAsset> = vec![];

    for total in liquid_asset.total_value_per_asset.clone() {
        let (available_amount, available_value) =
            get_info(&liquid_asset.available_asset_balance, &total.denom);
        let (in_order_amount, in_order_value) =
            get_info(&liquid_asset.in_orders_asset_balance, &total.denom);

        liquid_assets.push(LiquidAsset {
            denom: total.denom,
            price: total.price,
            available_amount,
            available_value,
            in_order_amount,
            in_order_value,
            total_amount: total.amount_token,
            total_value: total.amount_usdc,
        });
    }

    let liquid_eden = eden_to_liquid_asset(generator, &deps, user_address)?;
    liquid_assets.push(liquid_eden);

    Ok(GetLiquidAssetsResp {
        liquid_assets,
        total_liquid_asset_balance: liquid_asset.total_liquid_asset_balance.clone(),
    })
}

fn get_info(list_info: &Vec<CoinValue>, denom: &String) -> (Decimal, Decimal) {
    match list_info.iter().find(|info| &info.denom == denom).cloned() {
        Some(data) => (data.amount_token, data.amount_usdc),
        None => (Decimal::zero(), Decimal::zero()),
    }
}

fn eden_to_liquid_asset(generator: AccountSnapshotGenerator, &deps: &Deps<ElysQuery>, user_address: String) -> StdResult<LiquidAsset> {
    let querier = ElysQuerier::new(&deps.querier);

    let staked_assets = generator.get_staked_assets(&deps, &user_address)?;
    let eden_program = staked_assets.staked_assets.eden_earn_program;
    let available = eden_program.available.unwrap();
    let eden_coin_value = CoinValue::from_coin(
        &Coin::new(u128::from(available.amount),
        "ueden".to_string()),
    &querier, &generator.metadata.usdc_denom)?;

    Ok(LiquidAsset {
        denom: eden_coin_value.denom,
        price: eden_coin_value.price,
        available_amount: eden_coin_value.amount_token,
        available_value: eden_coin_value.amount_usdc,
        in_order_amount: Decimal::zero(),
        in_order_value: Decimal::zero(),
        total_amount: eden_coin_value.amount_token,
        total_value: eden_coin_value.amount_usdc,
    })
}