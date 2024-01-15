use std::collections::HashMap;

use cosmwasm_std::{
    BlockInfo, Coin, DecCoin, Decimal, Decimal256, QuerierWrapper, StdError, Uint128,
};
use cw_utils::Expiration;
use elys_bindings::trade_shield::{
    msg::{
        query_resp::{GetMarginOrdersResp, GetSpotOrdersResp},
        QueryMsg,
    },
    types::{MarginOrder, MarginOrderType, SpotOrder, Status, ElysDenom},
};

use crate::types::{AccountSnapshot, CoinValue};
use crate::action::query::get_eden_boost_earn_program_details;

use super::*;

pub fn update_account(deps: DepsMut<ElysQuery>, env: Env) -> StdResult<Response<ElysMsg>> {
    let querier = ElysQuerier::new(&deps.querier);
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let trade_shield_address = TRADE_SHIELD_ADDRESS.load(deps.storage)?;

    let mut pagination = PAGINATION.load(deps.storage)?;
    let expiration = EXPIRATION.load(deps.storage)?;

    let resp = querier
        .accounts(Some(pagination.clone()))
        .map_err(|err| custom_err(25, "Auth", err))?;

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
        let staked_assets = get_staked_assets(&deps, &address)?;

        let new_part: AccountSnapshot = create_new_part(
            &env.block,
            &querier,
            &expiration,
            account_balances,
            order_balances,
            staked_assets,
            &value_denom,
        )?;
        if let Some(part) = new_part {
            history.push(part);
        }
        if history.is_empty() {
            HISTORY.remove(deps.storage, &address);
        } else {
            HISTORY.save(deps.storage, &address, &history)?;
        }
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
) -> StdResult<Option<AccountSnapshot>> {
    let date = match expiration {
        Expiration::AtHeight(_) => Expiration::AtHeight(block.height),
        Expiration::AtTime(_) => Expiration::AtTime(block.time),
        Expiration::Never {} => panic!("never expire"),
    };

    let available_asset_balance: Vec<CoinValue> = account_balances
        .iter()
        .map(
            |coin| match CoinValue::from_coin(coin, querier, value_denom) {
                Ok(res) => res,
                Err(_) => CoinValue {
                    denom: coin.denom.to_owned(),
                    amount: Decimal::zero(),
                    price: Decimal::zero(),
                    value: Decimal::zero(),
                },
            },
        )
        .collect();

    let in_orders_asset_balance: Vec<CoinValue> = orders_balances
        .iter()
        .map(
            |coin| match CoinValue::from_coin(coin, querier, value_denom) {
                Ok(res) => res,
                Err(_) => CoinValue {
                    denom: coin.denom.to_owned(),
                    amount: Decimal::zero(),
                    price: Decimal::zero(),
                    value: Decimal::zero(),
                },
            },
        )
        .collect();

    let mut total_available_balance = DecCoin::new(Decimal256::zero(), value_denom);
    let mut total_in_orders_balance = DecCoin::new(Decimal256::zero(), value_denom);

    for balance in &available_asset_balance {
        total_available_balance.amount = total_available_balance
            .amount
            .checked_add(Decimal256::from(balance.value).clone())?
    }

    for balance in &in_orders_asset_balance {
        total_in_orders_balance.amount = total_in_orders_balance
            .amount
            .checked_add(Decimal256::from(balance.value).clone())?
    }

    let mut total_value_per_asset: HashMap<&String, CoinValue> = HashMap::new();

    for available in available_asset_balance.iter() {
        total_value_per_asset
            .entry(&available.denom)
            .and_modify(|e| {
                e.amount += available.amount;
                e.value = available.value;
            })
            .or_insert_with(|| available.clone());
    }

    for in_order in in_orders_asset_balance.iter() {
        total_value_per_asset
            .entry(&in_order.denom)
            .and_modify(|e| {
                e.amount += in_order.amount;
                e.value = in_order.value;
            })
            .or_insert_with(|| in_order.clone());
    }

    let total_value_per_asset: Vec<CoinValue> = total_value_per_asset.values().cloned().collect();

    let total_liquid_asset_balance = DecCoin::new(
        Decimal256::from(
            total_value_per_asset
                .iter()
                .map(|v| v.value)
                .fold(Decimal::zero(), |acc, item| acc + item),
        ),
        value_denom,
    );

    Ok(if total_liquid_asset_balance.amount.is_zero() {
        None
    } else {
        Some(AccountSnapshot {
            date,
            total_liquid_asset_balance,
            total_available_balance,
            total_in_orders_balance,
            available_asset_balance,
            in_orders_asset_balance,
            total_value_per_asset,
            staked_assets,
        })
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
                block_info.height > time + expiration
            }
            (Expiration::AtTime(time), Expiration::AtTime(expiration)) => {
                block_info.time.nanos() > time.nanos() + expiration.nanos()
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
        .map_err(|err| custom_err(136, "GetSpotOrders", err))?;
    let margin_order: GetMarginOrdersResp = querier
        .query_wasm_smart(
            trade_shield_address,
            &QueryMsg::GetMarginOrders {
                pagination: None,
                order_owner: Some(owner.clone()),
                order_type: Some(MarginOrderType::LimitOpen),
                order_status: Some(Status::Pending),
            },
        )
        .map_err(|err| custom_err(148, "GetMarginOrders", err))?;
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

pub fn custom_err(line: u64, module: &str, err: StdError) -> StdError {
    StdError::generic_err(format!(
        "at line :{line}\n when calling:{module}\n getting this error {err:?}"
    ))
}
pub fn get_staked_assets(
    deps: &DepsMut<ElysQuery>,
    address: &String,
) -> StdResult<Vec<Coin>> {
    get_eden_boost_earn_program_details(deps, Some(address.to_owned()), ElysDenom::EdenBoost.as_str().to_string())
}
