use std::collections::HashMap;

use cosmwasm_std::{BlockInfo, Coin, Decimal, QuerierWrapper, StdError, Uint128};
use cw_utils::Expiration;
use elys_bindings::query_resp::AmmSwapEstimationByDenomResponse;
use trade_shield_contract::{
    msg::{
        query_resp::{GetMarginOrdersResp, GetSpotOrdersResp},
        QueryMsg,
    },
    types::{MarginOrder, MarginOrderType, SpotOrder, Status},
};

use crate::types::{AccountSnapshot, CoinValue};

use super::*;

pub fn update_account(deps: DepsMut<ElysQuery>, env: Env) -> StdResult<Response<ElysMsg>> {
    let querier = ElysQuerier::new(&deps.querier);
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let trade_shield_address = TRADE_SHIELD_ADDRESS.load(deps.storage)?;

    let mut pagination = PAGINATION.load(deps.storage)?;
    let expiration = EXPIRATION.load(deps.storage)?;

    let resp = querier.accounts(Some(pagination.clone()))?;

    pagination.update(resp.pagination.next_key);
    PAGINATION.save(deps.storage, &pagination)?;

    for address in resp.addresses {
        let mut history = if let Some(history) = HISTORY.may_load(deps.storage, &address)? {
            update_history(history, &env.block, &expiration)
        } else {
            vec![]
        };
        let account_balances = deps.querier.query_all_balances(&address)?;
        let order_balances = get_all_order(&deps.querier, &trade_shield_address, &address)?;
        let new_part: AccountSnapshot = create_new_part(
            &env.block,
            &querier,
            &expiration,
            account_balances,
            order_balances,
            &value_denom,
        )?;
        history.push(new_part);
        HISTORY.save(deps.storage, &address, &history)?;
    }

    Ok(Response::default())
}

fn create_new_part(
    block: &BlockInfo,
    querier: &ElysQuerier<'_>,
    expiration: &Expiration,
    account_balances: Vec<Coin>,
    orders_balances: Vec<Coin>,
    value_denom: &String,
) -> StdResult<AccountSnapshot> {
    let date = match expiration {
        Expiration::AtHeight(_) => Expiration::AtHeight(block.height),
        Expiration::AtTime(_) => Expiration::AtTime(block.time),
        Expiration::Never {} => panic!("never expire"),
    };

    let mut account_value = Uint128::zero();
    let mut orders_value = Uint128::zero();

    let avaible_asset_balance = account_balances
        .iter()
        .map(|coin| CoinValue::from_coin(coin, querier, value_denom))
        .collect::<Result<Vec<CoinValue>, StdError>>()?;

    let in_orders_asset_balance = orders_balances
        .iter()
        .map(|coin| CoinValue::from_coin(coin, querier, value_denom))
        .collect::<Result<Vec<CoinValue>, StdError>>()?;

    let total_avaible_balance = avaible_asset_balance.iter().map(|balance| )

    Ok(AccountSnapshot {
        date,
        total_liquid_asset_balance: todo!(),
        total_avaible_balance: todo!(),
        total_in_orders_balance: todo!(),
        avaible_asset_balance,
        in_orders_asset_balance,
    })
}

fn update_history(
    history: Vec<AccountSnapshot>,
    block_info: &BlockInfo,
    expiration: &Expiration,
) -> Vec<AccountSnapshot> {
    let clean_history: Vec<AccountSnapshot> = history
        .into_iter()
        .filter(|history| match (history.date, expiration) {
            (Expiration::AtHeight(time), Expiration::AtHeight(expiration)) => {
                block_info.height < time + expiration
            }
            (Expiration::AtTime(time), Expiration::AtTime(expiration)) => {
                block_info.time.nanos() < time.nanos() + expiration.nanos()
            }
            _ => false,
        })
        .collect();

    clean_history
}

pub fn get_all_order(
    querier: &QuerierWrapper<ElysQuery>,
    trade_shield_address: &String,
    owner: &String,
) -> StdResult<Vec<Coin>> {
    let spot_order: GetSpotOrdersResp = querier.query_wasm_smart(
        trade_shield_address,
        &QueryMsg::GetSpotOrders {
            pagination: None,
            order_owner: Some(owner.clone()),
            order_type: None,
            order_status: Some(Status::Pending),
        },
    )?;
    let margin_order: GetMarginOrdersResp = querier.query_wasm_smart(
        trade_shield_address,
        &QueryMsg::GetMarginOrders {
            pagination: None,
            order_owner: Some(owner.clone()),
            order_type: Some(MarginOrderType::LimitOpen),
            order_status: Some(Status::Pending),
        },
    )?;
    let mut map: HashMap<String, Uint128> = HashMap::new();

    for SpotOrder { order_amount, .. } in spot_order.orders {
        map.entry(order_amount.denom)
            .and_modify(|e| *e += order_amount.amount)
            .or_insert(order_amount.amount);
    }

    for MarginOrder { collateral, .. } in margin_order.orders {
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
