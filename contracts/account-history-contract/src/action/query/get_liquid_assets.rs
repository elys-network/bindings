use cosmwasm_std::{DecCoin, Decimal, Decimal256, Deps, Env, StdResult};
use elys_bindings::{
    account_history::types::CoinValue,
    query_resp::{Entry, QueryGetEntryResponse},
    ElysQuerier, ElysQuery,
};

use crate::{
    msg::query_resp::{GetLiquidAssetsResp, LiquidAsset},
    states::{EXPIRATION, TRADE_SHIELD_ADDRESS},
    types::AccountSnapshotGenerator,
};

pub fn get_liquid_assets(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
) -> StdResult<GetLiquidAssetsResp> {
    let querier = ElysQuerier::new(&deps.querier);

    let expiration = EXPIRATION.load(deps.storage)?;
    let trade_shield_address = TRADE_SHIELD_ADDRESS.load(deps.storage)?;

    let QueryGetEntryResponse {
        entry: Entry {
            denom: usdc_denom, ..
        },
    } = querier.get_asset_profile("uusdc".to_string())?;

    let generator = AccountSnapshotGenerator::new(&querier, trade_shield_address, expiration)?;

    let snapshot = match generator.generate_account_snapshot_for_address(
        &querier,
        &deps,
        &env,
        &user_address,
    )? {
        Some(snapshot) => snapshot,
        None => {
            return Ok(GetLiquidAssetsResp {
                liquid_assets: vec![],
                total_liquid_asset_balance: DecCoin::new(Decimal256::zero(), usdc_denom),
            });
        }
    };

    let mut liquid_assets: Vec<LiquidAsset> = vec![];

    for total in snapshot.liquid_asset.total_value_per_asset.clone() {
        let (available_amount, available_value) =
            get_info(&snapshot.liquid_asset.available_asset_balance, &total.denom);
        let (in_order_amount, in_order_value) =
            get_info(&snapshot.liquid_asset.in_orders_asset_balance, &total.denom);

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

    Ok(GetLiquidAssetsResp {
        liquid_assets,
        total_liquid_asset_balance: snapshot.liquid_asset.total_liquid_asset_balance.clone(),
    })
}

fn get_info(list_info: &Vec<CoinValue>, denom: &String) -> (Decimal, Decimal) {
    match list_info.iter().find(|info| &info.denom == denom).cloned() {
        Some(data) => (data.amount_token, data.amount_usdc),
        None => (Decimal::zero(), Decimal::zero()),
    }
}
