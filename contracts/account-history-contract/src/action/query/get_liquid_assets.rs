use cosmwasm_std::{Decimal, Deps, Env, StdResult, Storage};
use elys_bindings::{account_history::types::CoinValue, ElysQuerier, ElysQuery};

use crate::{
    msg::query_resp::{GetLiquidAssetsResp, LiquidAsset},
    types::AccountSnapshotGenerator,
};

pub fn get_liquid_assets(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<GetLiquidAssetsResp> {
    let querier = ElysQuerier::new(&deps.querier);

    let generator = AccountSnapshotGenerator::new(&deps)?;

    let liquid_asset = generator.query_get_liquid_assets(&deps, &querier, &user_address)?;

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
            total_value: total.amount_usd,
        });
    }

    Ok(GetLiquidAssetsResp {
        liquid_assets,
        total_liquid_asset_balance: liquid_asset.total_liquid_asset_balance.clone(),
    })
}

fn get_info(list_info: &Vec<CoinValue>, denom: &String) -> (Decimal, Decimal) {
    list_info
        .iter()
        .find(|info| &info.denom == denom)
        .map_or((Decimal::zero(), Decimal::zero()), |data| {
            (data.amount_token, data.amount_usd)
        })
}
