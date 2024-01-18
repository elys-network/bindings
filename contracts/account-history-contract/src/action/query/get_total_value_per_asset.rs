use cosmwasm_std::{DecCoin, Decimal, Decimal256, Deps, StdResult};
use elys_bindings::ElysQuery;

use crate::{
    action::VALUE_DENOM,
    msg::query_resp::{GetLiquidAssetsResp, TotalValueOfAssetResp},
    states::HISTORY,
    types::CoinValue,
};

pub fn get_total_value_per_asset(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<GetLiquidAssetsResp> {
    let snapshot = match HISTORY.load(deps.storage, &user_address)?.last().cloned() {
        Some(snapshot) => snapshot,
        None => {
            let value_denom = VALUE_DENOM.load(deps.storage)?;
            return Ok(GetLiquidAssetsResp {
                liquid_assets: vec![],
                total_liquid_asset_balance: DecCoin::new(Decimal256::zero(), value_denom),
            });
        }
    };

    let mut liquid_assets: Vec<TotalValueOfAssetResp> = vec![];

    for total in snapshot.total_value_per_asset {
        let (available_amount, available_value) =
            get_info(&snapshot.in_orders_asset_balance, &total.denom);
        let (in_order_amount, in_order_value) =
            get_info(&snapshot.in_orders_asset_balance, &total.denom);

        liquid_assets.push(TotalValueOfAssetResp {
            denom: total.denom,
            price: total.price,
            available_amount,
            available_value,
            in_order_amount,
            in_order_value,
            total_amount: total.amount,
            total_value: total.value,
        });
    }

    Ok(GetLiquidAssetsResp {
        liquid_assets,
        total_liquid_asset_balance: snapshot.total_liquid_asset_balance,
    })
}

fn get_info(list_info: &Vec<CoinValue>, denom: &String) -> (Decimal, Decimal) {
    match list_info.iter().find(|info| &info.denom == denom).cloned() {
        Some(data) => (data.amount, data.value),
        None => (Decimal::zero(), Decimal::zero()),
    }
}
