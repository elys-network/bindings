use super::*;
use std::collections::HashMap;

use cosmwasm_std::{
    coin, BlockInfo, Coin, DecCoin, Decimal, Decimal256, Deps, QuerierWrapper, StdError, Uint128,
};
use cw_utils::Expiration;
use elys_bindings::{
    query_resp::AmmSwapEstimationByDenomResponse,
    trade_shield::{
        msg::{
            query_resp::{
                GetMarginOrdersResp, GetMarginPositionsForAddressResp, GetSpotOrdersResp,
            },
            QueryMsg,
        },
        types::{MarginOrder, MarginOrderType, SpotOrder, Status},
    },
};

use crate::{
    action::query::{
        get_eden_boost_earn_program_details, get_eden_earn_program_details,
        get_elys_earn_program_details, get_usdc_earn_program_details,
    },
    msg::query_resp::{GetRewardsResp, StakedAssetsResponse},
    types::{
        earn_program::{EdenBoostEarnProgram, EdenEarnProgram, ElysEarnProgram, UsdcEarnProgram},
        AccountSnapshot, CoinValue, ElysDenom, LiquidAsset, PerpetualAsset, PerpetualAssets,
        Portfolio, Reward, StakedAssets, TotalBalance,
    },
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
        let rewards_response = get_rewards(deps.as_ref(), address.clone())?;
        let perpetual_response = get_perpetuals(
            deps.as_ref(),
            trade_shield_address.clone(),
            &value_denom,
            address.clone(),
        )?;

        let new_part = create_new_part(
            &env.block,
            &querier,
            &expiration,
            account_balances,
            order_balances,
            staked_response,
            rewards_response,
            perpetual_response,
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
    staked_assets_resp: StakedAssetsResponse,
    rewards_response: GetRewardsResp,
    perpetual_response: PerpetualAssets,
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
                    amount: Decimal::from_atomics(coin.amount, 6 as u32).unwrap(),
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
                    amount: Decimal::from_atomics(coin.amount, 6 as u32).unwrap(),
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
                e.value += available.value.clone();
            })
            .or_insert_with(|| available.clone());
    }

    for in_order in in_orders_asset_balance.iter() {
        total_value_per_asset
            .entry(&in_order.denom)
            .and_modify(|e| {
                e.amount += in_order.amount.clone();
                e.value += in_order.value.clone();
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

    let reward = rewards_response.rewards;
    let portfolio_usd = DecCoin::new(
        total_liquid_asset_balance
            .amount
            .checked_add(Decimal256::from(
                staked_assets_resp.total_staked_balance.amount,
            ))?
            .checked_add(perpetual_response.total_perpetual_pools_balance.amount)?,
        value_denom,
    );
    let reward_usd: DecCoin = DecCoin::new(Decimal256::from(reward.clone().total_usd), value_denom);
    let total_balance = DecCoin::new(
        portfolio_usd.amount.checked_add(reward_usd.amount)?,
        value_denom,
    );

    // Adds the records all the time as we should return data to the FE even if it is 0 balanced.
    Ok(Some(AccountSnapshot {
        date,
        total_balance: TotalBalance {
            total_balance,
            portfolio_usd: portfolio_usd.clone(),
            reward_usd,
        },
        portfolio: Portfolio {
            balance_usd: portfolio_usd,
            liquid_assets_usd: total_liquid_asset_balance.clone(),
            staked_committed_usd: DecCoin::new(
                Decimal256::from(staked_assets_resp.total_staked_balance.amount),
                value_denom,
            ),
            liquidity_positions_usd: DecCoin::new(Decimal256::zero(), value_denom),
            leverage_lp_usd: DecCoin::new(Decimal256::zero(), value_denom),
            margin_usd: DecCoin::new(Decimal256::zero(), value_denom),
            usdc_earn_usd: DecCoin::new(Decimal256::zero(), value_denom),
            borrows_usd: DecCoin::new(Decimal256::zero(), value_denom),
        },
        reward,
        liquid_asset: LiquidAsset {
            total_liquid_asset_balance,
            total_available_balance,
            total_in_orders_balance,
            available_asset_balance,
            in_orders_asset_balance,
            total_value_per_asset,
        },
        staked_assets: staked_assets_resp.staked_assets,
        perpetual_assets: perpetual_response,
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
) -> StakedAssetsResponse {
    // create staked_assets variable that is a StakedAssets struct
    let mut staked_assets = StakedAssets::default();
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
    let staked_asset_usdc = usdc_details.data.clone();
    total_balance = total_balance
        .checked_add(match staked_asset_usdc.clone() {
            UsdcEarnProgram {
                staked: Some(r), ..
            } => r.usd_amount,
            _ => Decimal::zero(),
        })
        .unwrap();
    staked_assets.usdc_earn_program = staked_asset_usdc;

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
    let staked_asset_elys = elys_details.data;
    total_balance = total_balance
        .checked_add(match staked_asset_elys.clone() {
            ElysEarnProgram {
                staked: Some(r), ..
            } => r.usd_amount,
            _ => Decimal::zero(),
        })
        .unwrap();
    staked_assets.elys_earn_program = staked_asset_elys;

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
    let staked_asset_eden = eden_details.data;
    total_balance = total_balance
        .checked_add(match staked_asset_eden.clone() {
            EdenEarnProgram {
                staked: Some(r), ..
            } => r.usd_amount,
            _ => Decimal::zero(),
        })
        .unwrap();
    staked_assets.eden_earn_program = staked_asset_eden;

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
    let staked_asset_edenb = edenb_details.data;
    total_balance = total_balance
        .checked_add(match staked_asset_edenb.clone() {
            EdenBoostEarnProgram {
                rewards: Some(r), ..
            } => r.iter().fold(Decimal::zero(), |acc, item| {
                acc.checked_add(item.usd_amount.unwrap()).unwrap()
            }),
            _ => Decimal::zero(),
        })
        .unwrap();
    staked_assets.eden_boost_earn_program = staked_asset_edenb;

    StakedAssetsResponse {
        staked_assets: staked_assets,
        total_staked_balance: DecCoin::new(Decimal256::from(total_balance), usdc_denom),
    }
}

pub fn get_rewards(deps: Deps<ElysQuery>, address: String) -> StdResult<GetRewardsResp> {
    let querier = ElysQuerier::new(&deps.querier);
    let commitments = querier.get_commitments(address)?;

    let denom_usdc_entry = querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
    let denom_uusdc = denom_usdc_entry.entry.denom;
    let usdc_display_denom = denom_usdc_entry.entry.display_name;

    let denom_uelys = ElysDenom::Elys.as_str().to_string();
    let denom_ueden = ElysDenom::Eden.as_str().to_string();
    let denom_uedenb = ElysDenom::EdenBoost.as_str().to_string();

    let usdc_oracle_price = querier.get_oracle_price(
        usdc_display_denom.clone(),
        ElysDenom::AnySource.as_str().to_string(),
        0,
    )?;
    let usdc_price = usdc_oracle_price
        .price
        .price
        .checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap())
        .unwrap();

    let mut rewards = Reward {
        usdc_usd: Decimal::zero(),
        eden_usd: Decimal::zero(),
        eden_boost: Uint128::zero(),
        other_usd: Decimal::zero(),
        total_usd: Decimal::zero(),
    };

    match commitments.commitments.rewards_unclaimed {
        Some(rewards_unclaimed) => {
            for reward in rewards_unclaimed {
                // uusdc
                if reward.denom == denom_uusdc {
                    let usdc_rewards = Decimal::from_atomics(reward.amount, 0).unwrap();
                    rewards.usdc_usd = usdc_rewards.checked_mul(usdc_price).unwrap();
                    rewards.total_usd = rewards.total_usd.checked_add(rewards.usdc_usd).unwrap();

                    continue;
                }

                // ueden
                if reward.denom == denom_ueden {
                    // if it is eden, we should elys denom instead of ueden as it is not available in LP pool and has the same value with elys.
                    let reward_in_elys = coin(reward.amount.u128(), denom_uelys.to_owned());
                    let AmmSwapEstimationByDenomResponse { amount, .. } = querier
                        .amm_swap_estimation_by_denom(
                            &reward_in_elys,
                            denom_uelys.to_owned(),
                            denom_uusdc.to_owned(),
                            &Decimal::zero(),
                        )?;
                    let rewards_in_usdc = Decimal::from_atomics(amount.amount, 0).unwrap();
                    rewards.eden_usd = rewards_in_usdc.checked_mul(usdc_price).unwrap();
                    rewards.total_usd = rewards.total_usd.checked_add(rewards.eden_usd).unwrap();

                    continue;
                }

                // uedenb - we don't value eden boost in usd.
                if reward.denom == denom_uedenb {
                    rewards.eden_boost = reward.amount;
                    continue;
                }

                // We accumulate other denoms in a single usd.
                let AmmSwapEstimationByDenomResponse { amount, .. } = querier
                    .amm_swap_estimation_by_denom(
                        &reward,
                        reward.denom.to_owned(),
                        &denom_uusdc.to_owned(),
                        &Decimal::zero(),
                    )?;
                let rewards_in_usdc = Decimal::from_atomics(amount.amount, 0).unwrap();
                let rewards_in_usd = rewards_in_usdc.checked_mul(usdc_price).unwrap();

                rewards.other_usd = rewards.other_usd.checked_add(rewards_in_usd).unwrap();
                rewards.total_usd = rewards.total_usd.checked_add(rewards_in_usd).unwrap();
            }
        }
        None => {
            let value_denom = VALUE_DENOM.load(deps.storage)?;
            return Ok(GetRewardsResp {
                rewards: AccountSnapshot::zero(&value_denom).reward,
            });
        }
    }

    let resp = GetRewardsResp { rewards: rewards };
    Ok(resp)
}

fn get_perpetuals(
    deps: Deps<ElysQuery>,
    trade_shield_address: String,
    value_denom: &String,
    address: String,
) -> StdResult<PerpetualAssets> {
    let GetMarginPositionsForAddressResp { mtps, .. } = deps.querier.query_wasm_smart(
        trade_shield_address,
        &QueryMsg::MarginGetPositionsForAddress {
            address,
            pagination: None,
        },
    )?;
    let mut perpetual_vec: Vec<PerpetualAsset> = vec![];
    let querier = ElysQuerier::new(&deps.querier);

    for mtp in mtps {
        match PerpetualAsset::new(mtp, value_denom.to_owned(), &querier) {
            Ok(perpetual_asset) => perpetual_vec.push(perpetual_asset),
            Err(_) => continue,
        }
    }

    let total_perpetual_pools_balance_amount = perpetual_vec
        .iter()
        .map(|perpetual| perpetual.size.amount)
        .fold(Decimal256::zero(), |acc, item| acc + item);
    let total_perpetual_pools_balance =
        DecCoin::new(total_perpetual_pools_balance_amount, value_denom.to_owned());

    Ok(PerpetualAssets {
        total_perpetual_pools_balance,
        perpetual_asset: perpetual_vec,
    })
}
