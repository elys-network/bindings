use super::*;
use std::collections::HashMap;

use cosmwasm_std::{
    coin, BlockInfo, Coin, DecCoin, Decimal, Decimal256, QuerierWrapper, StdError, Uint128,
};
use cw_utils::Expiration;
use elys_bindings::trade_shield::{
    msg::{
        query_resp::{GetMarginOrdersResp, GetSpotOrdersResp},
        QueryMsg,
    },
    types::{MarginOrder, MarginOrderType, SpotOrder, Status},
};

use crate::{
    action::query::{
        get_eden_boost_earn_program_details, get_eden_earn_program_details,
        get_elys_earn_program_details, get_usdc_earn_program_details,
    },
    types::{AccountSnapshot, CoinValue, ElysDenom, StakedAsset, StakedAssetResponse},
};
use elys_bindings::{query_resp::QueryAprResponse, types::EarnType};

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

    // Read common variables before looping
    // To enhance querying speed.
    let usdc_denom_entry = querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
    let usdc_denom = usdc_denom_entry.entry.denom;
    let usdc_display_denom = usdc_denom_entry.entry.display_name;
    let usdc_decimal = u64::checked_pow(10, usdc_denom_entry.entry.decimals as u32).unwrap();

    let eden_denom_entry = querier.get_asset_profile(ElysDenom::Eden.as_str().to_string())?;
    let eden_decimal = u64::checked_pow(10, eden_denom_entry.entry.decimals as u32).unwrap();

    let discount = Decimal::zero();
    let usdc_oracle_price = querier.get_oracle_price(
        usdc_display_denom.clone(),
        ElysDenom::AnySource.as_str().to_string(),
        0,
    )?;
    let uusdc_usd_price = usdc_oracle_price
        .price
        .price
        .checked_div(Decimal::from_atomics(Uint128::new(usdc_decimal as u128), 0).unwrap())
        .unwrap();
    let uelys_price_in_uusdc = querier.get_amm_price_by_denom(
        coin(
            Uint128::new(1000000).u128(),
            ElysDenom::Elys.as_str().to_string(),
        ),
        discount,
    )?;

    // APR section
    let usdc_apr_usdc = querier.get_incentive_apr(
        EarnType::UsdcProgram as i32,
        ElysDenom::Usdc.as_str().to_string(),
    )?;
    let eden_apr_usdc = querier.get_incentive_apr(
        EarnType::UsdcProgram as i32,
        ElysDenom::Eden.as_str().to_string(),
    )?;

    let usdc_apr_edenb = querier.get_incentive_apr(
        EarnType::EdenBProgram as i32,
        ElysDenom::Usdc.as_str().to_string(),
    )?;
    let eden_apr_edenb = querier.get_incentive_apr(
        EarnType::EdenBProgram as i32,
        ElysDenom::Eden.as_str().to_string(),
    )?;

    let usdc_apr_eden = querier.get_incentive_apr(
        EarnType::EdenProgram as i32,
        ElysDenom::Usdc.as_str().to_string(),
    )?;
    let eden_apr_eden = querier.get_incentive_apr(
        EarnType::EdenProgram as i32,
        ElysDenom::Eden.as_str().to_string(),
    )?;
    let edenb_apr_eden = querier.get_incentive_apr(
        EarnType::EdenProgram as i32,
        ElysDenom::EdenBoost.as_str().to_string(),
    )?;

    let usdc_apr_elys = querier.get_incentive_apr(
        EarnType::ElysProgram as i32,
        ElysDenom::Usdc.as_str().to_string(),
    )?;
    let eden_apr_elys = querier.get_incentive_apr(
        EarnType::ElysProgram as i32,
        ElysDenom::Eden.as_str().to_string(),
    )?;
    let edenb_apr_elys = querier.get_incentive_apr(
        EarnType::ElysProgram as i32,
        ElysDenom::EdenBoost.as_str().to_string(),
    )?;

    for address in resp.addresses {
        let mut history = if let Some(history) = HISTORY.may_load(deps.storage, &address)? {
            update_history(history, &env.block, &expiration)
        } else {
            vec![]
        };
        let account_balances = deps.querier.query_all_balances(&address)?;
        let order_balances = get_all_order(&deps.querier, &trade_shield_address, &address)?;
        let staked_response = get_staked_assets(
            &deps,
            &address,
            uusdc_usd_price,
            uelys_price_in_uusdc,
            usdc_denom.to_owned(),
            eden_decimal,
            usdc_apr_usdc.to_owned(),
            eden_apr_usdc.to_owned(),
            usdc_apr_edenb.to_owned(),
            eden_apr_edenb.to_owned(),
            usdc_apr_eden.to_owned(),
            eden_apr_eden.to_owned(),
            edenb_apr_eden.to_owned(),
            usdc_apr_elys.to_owned(),
            eden_apr_elys.to_owned(),
            edenb_apr_elys.to_owned(),
        );

        let new_part = create_new_part(
            &env.block,
            &querier,
            &expiration,
            account_balances,
            order_balances,
            staked_response,
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
    staked_assets_resp: StakedAssetResponse,
    value_denom: &String,
) -> StdResult<Option<AccountSnapshot>> {
    let date = match expiration {
        Expiration::AtHeight(_) => Expiration::AtHeight(block.height),
        Expiration::AtTime(_) => Expiration::AtTime(block.time),
        Expiration::Never {} => panic!("never expire"),
    };

    let mut a = false;

    let available_asset_balance: Vec<CoinValue> = account_balances
        .iter()
        .map(
            |coin| match CoinValue::from_coin(coin, querier, value_denom) {
                Ok(res) => {
                    a = true;
                    res
                }
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
            .checked_add(Decimal256::from(balance.value.clone()))?
    }

    for balance in &in_orders_asset_balance {
        total_in_orders_balance.amount = total_in_orders_balance
            .amount
            .checked_add(Decimal256::from(balance.value.clone()))?
    }

    let mut total_value_per_asset: HashMap<&String, CoinValue> = HashMap::new();

    for available in available_asset_balance.iter() {
        total_value_per_asset
            .entry(&available.denom)
            .and_modify(|e| {
                e.amount += available.amount.clone();
                e.value = available.value.clone();
            })
            .or_insert_with(|| available.clone());
    }

    for in_order in in_orders_asset_balance.iter() {
        total_value_per_asset
            .entry(&in_order.denom)
            .and_modify(|e| {
                e.amount += in_order.amount.clone();
                e.value = in_order.value.clone();
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

    // Adds the records all the time as we should return data to the FE even if it is 0 balanced.
    Ok(Some(AccountSnapshot {
        date,
        total_liquid_asset_balance,
        total_available_balance,
        total_in_orders_balance,
        available_asset_balance,
        in_orders_asset_balance,
        total_value_per_asset,
        total_staked_asset_balance: staked_assets_resp.total_balance,
        staked_assets: staked_assets_resp.staked_assets,
    }))
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
    uusdc_usd_price: Decimal,
    uelys_price_in_uusdc: Decimal,
    usdc_denom: String,
    eden_decimal: u64,
    usdc_apr_usdc: QueryAprResponse,
    eden_apr_usdc: QueryAprResponse,
    usdc_apr_edenb: QueryAprResponse,
    eden_apr_edenb: QueryAprResponse,
    usdc_apr_eden: QueryAprResponse,
    eden_apr_eden: QueryAprResponse,
    edenb_apr_eden: QueryAprResponse,
    usdc_apr_elys: QueryAprResponse,
    eden_apr_elys: QueryAprResponse,
    edenb_apr_elys: QueryAprResponse,
) -> StakedAssetResponse {
    let mut staked_assets: Vec<StakedAsset> = Vec::new();
    let mut total_balance = Decimal::zero();

    let usdc_details = get_usdc_earn_program_details(
        deps,
        Some(address.to_owned()),
        ElysDenom::Usdc.as_str().to_string(),
        usdc_denom.to_owned(),
        uusdc_usd_price,
        uelys_price_in_uusdc,
        usdc_apr_usdc,
        eden_apr_usdc,
    )
    .unwrap();
    // usdc program
    let staked_asset_usdc = StakedAsset {
        program: EarnType::UsdcProgram,
        apr: Decimal::from_atomics(usdc_details.data.apr.ueden, 0).unwrap(),
        available: match usdc_details.data.available {
            Some(r) => r.usd_amount,
            None => Decimal::zero(),
        },
        rewards: match usdc_details.data.rewards {
            Some(r) => r.iter().map(|f| f.usd_amount.unwrap()).sum(),
            None => Decimal::zero(),
        },
        staked: match usdc_details.data.staked {
            Some(r) => r.usd_amount,
            None => Decimal::zero(),
        },
    };
    total_balance = total_balance.checked_add(staked_asset_usdc.staked).unwrap();
    staked_assets.push(staked_asset_usdc);

    // elys program
    let elys_details = get_elys_earn_program_details(
        deps,
        Some(address.to_owned()),
        ElysDenom::Elys.as_str().to_string(),
        usdc_denom.to_owned(),
        uusdc_usd_price,
        uelys_price_in_uusdc,
        usdc_apr_elys,
        eden_apr_elys,
        edenb_apr_elys,
    )
    .unwrap();
    let staked_asset_elys = StakedAsset {
        program: EarnType::ElysProgram,
        apr: Decimal::from_atomics(elys_details.data.apr.ueden, 0).unwrap(),
        available: match elys_details.data.available {
            Some(r) => r.usd_amount,
            None => Decimal::zero(),
        },
        rewards: match elys_details.data.rewards {
            Some(r) => r
                .iter()
                .filter(|f| f.usd_amount != None)
                .map(|f| f.usd_amount.unwrap())
                .sum(),
            None => Decimal::zero(),
        },
        staked: match elys_details.data.staked {
            Some(r) => r.usd_amount,
            None => Decimal::zero(),
        },
    };
    total_balance = total_balance
        .checked_add(staked_asset_elys.available)
        .unwrap();
    total_balance = total_balance
        .checked_add(staked_asset_elys.rewards)
        .unwrap();
    total_balance = total_balance.checked_add(staked_asset_elys.staked).unwrap();
    staked_assets.push(staked_asset_elys);

    // eden program
    let eden_details = get_eden_earn_program_details(
        deps,
        Some(address.to_owned()),
        ElysDenom::Eden.as_str().to_string(),
        usdc_denom.to_owned(),
        uusdc_usd_price,
        uelys_price_in_uusdc,
        usdc_apr_eden,
        eden_apr_eden,
        edenb_apr_eden,
    )
    .unwrap();
    let staked_asset_eden = StakedAsset {
        program: EarnType::EdenProgram,
        apr: Decimal::from_atomics(eden_details.data.apr.ueden, 0).unwrap(),
        available: match eden_details.data.available {
            Some(r) => r.usd_amount,
            None => Decimal::zero(),
        },
        rewards: match eden_details.data.rewards {
            Some(r) => r
                .iter()
                .filter(|f| f.usd_amount != None)
                .map(|f| f.usd_amount.unwrap())
                .sum(),
            None => Decimal::zero(),
        },
        staked: match eden_details.data.staked {
            Some(r) => r.usd_amount,
            None => Decimal::zero(),
        },
    };
    total_balance = total_balance
        .checked_add(staked_asset_eden.available)
        .unwrap();
    total_balance = total_balance
        .checked_add(staked_asset_eden.rewards)
        .unwrap();
    total_balance = total_balance.checked_add(staked_asset_eden.staked).unwrap();
    staked_assets.push(staked_asset_eden);

    let edenb_details = get_eden_boost_earn_program_details(
        deps,
        Some(address.to_owned()),
        ElysDenom::EdenBoost.as_str().to_string(),
        usdc_denom.to_owned(),
        uusdc_usd_price,
        uelys_price_in_uusdc,
        eden_decimal,
        usdc_apr_edenb,
        eden_apr_edenb,
    )
    .unwrap();
    let staked_asset_edenb = StakedAsset {
        program: EarnType::EdenBProgram,
        apr: Decimal::from_atomics(edenb_details.data.apr.ueden, 0).unwrap(),
        available: match edenb_details.data.available {
            Some(r) => Decimal::from_atomics(r, 0).unwrap(),
            None => Decimal::zero(),
        },
        rewards: match edenb_details.data.rewards {
            Some(r) => r.iter().map(|f| f.usd_amount.unwrap()).sum(),
            None => Decimal::zero(),
        },
        staked: match edenb_details.data.staked {
            Some(r) => Decimal::from_atomics(r, 0).unwrap(),
            None => Decimal::zero(),
        },
    };
    total_balance = total_balance
        .checked_add(staked_asset_edenb.rewards)
        .unwrap();
    staked_assets.push(staked_asset_edenb);

    StakedAssetResponse {
        staked_assets: staked_assets,
        total_balance: total_balance,
    }
}
