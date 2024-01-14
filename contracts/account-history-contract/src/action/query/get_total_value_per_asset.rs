use cosmwasm_std::{Decimal, Deps, StdError, StdResult};
use elys_bindings::ElysQuery;

use crate::{
    msg::query_resp::{TotalValueOfAssetResp, TotalValuePerAssetResp},
    states::HISTORY,
    types::CoinValue,
};

pub fn get_total_value_per_asset(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<TotalValuePerAssetResp> {
    let snapshot = match HISTORY.load(deps.storage, &user_address)?.last().cloned() {
        Some(snapshot) => snapshot,
        None => return Err(StdError::not_found("account snapshot")),
    };

    let mut list_asset_value: Vec<TotalValueOfAssetResp> = vec![];

    for total in snapshot.total_value_per_asset {
        let (available_amount, available_value) =
            get_info(&snapshot.in_orders_asset_balance, &total.denom);
        let (in_order_amount, in_order_value) =
            get_info(&snapshot.in_orders_asset_balance, &total.denom);

        list_asset_value.push(TotalValueOfAssetResp {
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

    Ok(TotalValuePerAssetResp { list_asset_value })
}

fn get_info(list_info: &Vec<CoinValue>, denom: &String) -> (Decimal, Decimal) {
    match list_info.iter().find(|info| &info.denom == denom).cloned() {
        Some(data) => (data.amount, data.value),
        None => (Decimal::zero(), Decimal::zero()),
    }
}
