use std::collections::HashMap;

use cosmwasm_std::{Coin, QuerierWrapper, StdError, StdResult, Uint128};
use elys_bindings::{
    trade_shield::{
        msg::{query_resp::GetPerpetualOrdersResp, query_resp::GetSpotOrdersResp, QueryMsg},
        types::{PerpetualOrder, PerpetualOrderType, SpotOrder, Status},
    },
    ElysQuery,
};

pub fn get_all_orders(
    querier: &QuerierWrapper<ElysQuery>,
    trade_shield_address: &Option<String>,
    owner: &String,
) -> StdResult<Vec<Coin>> {
    let trade_shield_address = match trade_shield_address {
        Some(trade_shield_address) => trade_shield_address,
        None => return Ok(vec![]),
    };

    let spot_order: GetSpotOrdersResp = querier
        .query_wasm_smart(
            trade_shield_address,
            &QueryMsg::GetSpotOrders {
                pagination: None,
                order_owner: Some(owner.clone()),
                order_type: None,
                order_status: Some(Status::Pending),
            },
        )
        .map_err(|e| StdError::generic_err(format!("GetSpotOrders failed {}", e)))?;
    let perpetual_order: GetPerpetualOrdersResp = querier
        .query_wasm_smart(
            trade_shield_address,
            &QueryMsg::GetPerpetualOrders {
                pagination: None,
                order_owner: Some(owner.clone()),
                order_type: Some(PerpetualOrderType::LimitOpen),
                order_status: Some(Status::Pending),
            },
        )
        .map_err(|e| StdError::generic_err(format!("GetPerpetualOrders failed {}", e)))?;
    let mut map: HashMap<String, Uint128> = HashMap::new();

    for SpotOrder { order_amount, .. } in spot_order.orders {
        map.entry(order_amount.denom)
            .and_modify(|e| *e += order_amount.amount)
            .or_insert(order_amount.amount);
    }

    for PerpetualOrder { collateral, .. } in perpetual_order.orders {
        map.entry(collateral.denom)
            .and_modify(|e| *e += collateral.amount)
            .or_insert(collateral.amount);
    }

    let consolidated_coins: Vec<Coin> = map
        .into_iter()
        .map(|(denom, amount)| Coin { denom, amount })
        .collect();
    Ok(consolidated_coins)
}
