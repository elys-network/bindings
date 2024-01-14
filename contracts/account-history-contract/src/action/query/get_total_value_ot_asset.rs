use cosmwasm_std::{Decimal, Deps, StdError, StdResult};
use elys_bindings::ElysQuery;

use crate::{msg::query_resp::TotalValueOfAssetResp, states::HISTORY};

pub fn get_total_value_of_asset(
    deps: Deps<ElysQuery>,
    user_address: String,
    denom: String,
) -> StdResult<TotalValueOfAssetResp> {
    let snapshot = match HISTORY.load(deps.storage, &user_address)?.last().cloned() {
        Some(snapshot) => snapshot,
        None => return Err(StdError::not_found("account snapshot")),
    };

    let (total_amount, total_value, price) = match snapshot
        .total_value_per_asset
        .iter()
        .find(|snap| &snap.denom == &denom)
        .cloned()
    {
        Some(total) => (total.amount, total.value, total.price),
        None => return Err(StdError::not_found(format!("denom : {denom}"))),
    };

    let (in_order_amount, in_order_value) = match snapshot
        .in_orders_asset_balance
        .iter()
        .find(|snap| &snap.denom == &denom)
    {
        Some(in_order) => (in_order.amount, in_order.value),
        None => (Decimal::zero(), Decimal::zero()),
    };

    let (available_amount, available_value) = match snapshot
        .available_asset_balance
        .iter()
        .find(|snap| &snap.denom == &denom)
    {
        Some(available) => (available.amount, available.value),
        None => (Decimal::zero(), Decimal::zero()),
    };

    Ok(TotalValueOfAssetResp {
        denom,
        price,
        available_amount,
        available_value,
        in_order_amount,
        in_order_value,
        total_amount,
        total_value,
    })
}
