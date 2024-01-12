use std::collections::HashMap;

use cosmwasm_std::{BlockInfo, Coin, Decimal, QuerierWrapper, Uint128};
use cw_utils::Expiration;
use elys_bindings::query_resp::AmmSwapEstimationByDenomResponse;
use trade_shield_contract::{
    msg::{
        query_resp::{GetMarginOrdersResp, GetSpotOrdersResp},
        QueryMsg,
    },
    types::{MarginOrder, MarginOrderType, SpotOrder, Status},
};

use financial_snapshot_contract::{
    bindings::query_resp::QueryShowCommitmentsResponse,
    msg::query_resp::earn::GetUsdcPriceResp,
    msg::QueryMsg as FinancialQueryMsg,
    types::ElysDenom,
};
use crate::types::{AccountSnapshot, Rewards};

use super::*;

pub fn update_account(deps: DepsMut<ElysQuery>, env: Env) -> StdResult<Response<ElysMsg>> {
    let querier = ElysQuerier::new(&deps.querier);
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let trade_shield_address = TRADE_SHIELD_ADDRESS.load(deps.storage)?;
    let financial_address = FINANCIAL_SNAPSHOT_ADDRESS.load(deps.storage)?;

    let mut pagination = PAGINATION.load(deps.storage)?;
    let expiration = EXPIRATION.load(deps.storage)?;

    let resp = querier.accounts(Some(pagination.clone()))?;

    pagination.update(resp.pagination.next_key);
    PAGINATION.save(deps.storage, &pagination)?;

    let usdc_price: GetUsdcPriceResp = deps.querier.query_wasm_smart(
        &financial_address,
        &FinancialQueryMsg::GetUsdcPrice { },
    )?;

    for address in resp.addresses {
        let mut history = if let Some(history) = HISTORY.may_load(deps.storage, &address)? {
            update_history(history, &env.block, &expiration)
        } else {
            vec![]
        };
        let account_balances = deps.querier.query_all_balances(&address)?;
        let order_balances = get_all_order(&deps.querier, &trade_shield_address, &address)?;
        let rewards = get_all_rewards(&deps.querier, &financial_address, &address, &value_denom, &usdc_price.price)?;
        let new_part: AccountSnapshot = create_new_part(
            &env.block,
            &querier,
            &expiration,
            account_balances,
            order_balances,
            &value_denom,
            rewards,
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
    rewards: Rewards,
) -> StdResult<AccountSnapshot> {
    let date = match expiration {
        Expiration::AtHeight(_) => Expiration::AtHeight(block.height),
        Expiration::AtTime(_) => Expiration::AtTime(block.time),
        Expiration::Never {} => panic!("never expire"),
    };

    let mut account_value = Uint128::zero();
    let mut orders_value = Uint128::zero();

    for balance in &account_balances {
        if &balance.denom == value_denom {
            account_value += balance.amount;
            continue;
        }
        let AmmSwapEstimationByDenomResponse { amount, .. } = querier
            .amm_swap_estimation_by_denom(
                &balance,
                &balance.denom,
                value_denom,
                &Decimal::zero(),
            )?;
        account_value += amount.amount;
    }

    for balance in &orders_balances {
        if &balance.denom == value_denom {
            orders_value += balance.amount;
            continue;
        }
        let AmmSwapEstimationByDenomResponse { amount, .. } = querier
            .amm_swap_estimation_by_denom(
                &balance,
                &balance.denom,
                value_denom,
                &Decimal::zero(),
            )?;
        orders_value += amount.amount;
    }

    Ok(AccountSnapshot {
        date,
        account_value: Coin {
            denom: value_denom.to_owned(),
            amount: account_value,
        },
        locked_value: Coin {
            denom: value_denom.to_owned(),
            amount: orders_value,
        },
        account_assets: account_balances,
        locked_asset: orders_balances,
        rewards: rewards,
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

pub fn get_all_rewards(
    querier: &QuerierWrapper<ElysQuery>,
    financial_address: &String,
    user_address: &String,
    value_denom: &String,
    usdc_price: &Decimal,
) -> StdResult<Rewards> {
    let commitments: QueryShowCommitmentsResponse = querier.query_wasm_smart(
        financial_address,
        &FinancialQueryMsg::GetCommitments { delegator_addr: user_address.to_owned() },
    )?;
    
    let elys_querier = ElysQuerier::new(querier);

    let denom_usdc_entry = elys_querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
    let denom_uusdc = denom_usdc_entry.entry.denom;

    let denom_ueden = ElysDenom::Eden.as_str().to_string();
    let denom_uedenb = ElysDenom::EdenBoost.as_str().to_string();
    
    let mut rewards = Rewards {
        usdc: Decimal::zero(),
        eden:Decimal::zero(),
        edenb:Uint128::zero(),
        other:Decimal::zero(),
    };

    match commitments.commitments.rewards_unclaimed {
        Some(rewards_unclaimed) => {
            for reward in rewards_unclaimed { 
                if reward.denom == denom_uusdc {
                    let usdc_rewards = Decimal::from_atomics(reward.amount, 0).unwrap();
                    rewards.usdc = usdc_rewards.checked_mul(*usdc_price).unwrap();
                    break;
                }
                
                if reward.denom == denom_ueden {
                    let AmmSwapEstimationByDenomResponse { amount, .. } = elys_querier
                    .amm_swap_estimation_by_denom(
                        &reward,
                        reward.denom.to_owned(),
                        value_denom,
                        &Decimal::zero(),
                    )?;
                    let rewards_in_usdc = Decimal::from_atomics(amount.amount, 0).unwrap();
                    rewards.eden = rewards_in_usdc.checked_mul(*usdc_price).unwrap();
                    break;

                }
                if reward.denom == denom_uedenb {
                    rewards.edenb = reward.amount;
                    break;
                }
                
                // Extra denoms
                let AmmSwapEstimationByDenomResponse { amount, .. } = elys_querier
                .amm_swap_estimation_by_denom(
                    &reward,
                    reward.denom.to_owned(),
                    value_denom,
                    &Decimal::zero(),
                )?;
                let rewards_in_usdc = Decimal::from_atomics(amount.amount, 0).unwrap();
                let rewards_in_usd = rewards_in_usdc.checked_mul(*usdc_price).unwrap();

                rewards.other = rewards.other.checked_add(rewards_in_usd).unwrap();
            }
            Ok(rewards)
        },
        None =>  Ok(rewards),
    }
}