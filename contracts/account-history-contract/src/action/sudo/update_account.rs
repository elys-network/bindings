use std::collections::HashMap;

use chrono::Days;
use cosmwasm_std::{
    coin, BlockInfo, Coin, DecCoin, Decimal, Decimal256, DepsMut, Env, Response, StdError,
    StdResult, Timestamp, Uint128,
};
use cw_utils::Expiration;

use crate::{
    action::sudo::{
        get_all_orders::get_all_orders, get_perpetuals::get_perpetuals, get_rewards::get_rewards,
        get_staked_assets::get_staked_assets,
    },
    msg::query_resp::{GetRewardsResp, StakedAssetsResponse},
    states::{EXPIRATION, HISTORY, PAGINATION, TRADE_SHIELD_ADDRESS},
    types::{
        AccountSnapshot, CoinValue, ElysDenom, LiquidAsset, PerpetualAssets, Portfolio,
        TotalBalance,
    },
    utils::{get_raw_today, get_today},
};
use elys_bindings::{types::EarnType, ElysMsg, ElysQuerier, ElysQuery};

pub fn update_account(deps: DepsMut<ElysQuery>, env: Env) -> StdResult<Response<ElysMsg>> {
    let querier = ElysQuerier::new(&deps.querier);

    let trade_shield_address = TRADE_SHIELD_ADDRESS.load(deps.storage)?;

    let mut pagination = PAGINATION.load(deps.storage)?;
    let expiration = EXPIRATION.load(deps.storage)?;

    let resp = querier.accounts(Some(pagination.clone())).map_err(|e| {
        StdError::generic_err(format!(
            "failed to get accounts with pagination {:?}: {}",
            pagination, e
        ))
    })?;

    pagination.update(resp.pagination.next_key);
    PAGINATION.save(deps.storage, &pagination)?;

    // Read common variables before looping
    // To enhance querying speed.
    let usdc_denom_entry = querier
        .get_asset_profile(ElysDenom::Usdc.as_str().to_string())
        .map_err(|_| StdError::generic_err("an error occurred while getting usdc denom"))?;
    let usdc_denom = usdc_denom_entry.entry.denom;
    let usdc_base_denom = usdc_denom_entry.entry.base_denom;
    let usdc_display_denom = usdc_denom_entry.entry.display_name;
    let usdc_decimal = u64::checked_pow(10, usdc_denom_entry.entry.decimals as u32).unwrap();

    let eden_denom_entry = querier
        .get_asset_profile(ElysDenom::Eden.as_str().to_string())
        .map_err(|_| StdError::generic_err("an error occurred while getting eden denom"))?;
    let eden_decimal = u64::checked_pow(10, eden_denom_entry.entry.decimals as u32).unwrap();

    let discount = Decimal::zero();
    let usdc_oracle_price = querier
        .get_oracle_price(
            usdc_display_denom.clone(),
            ElysDenom::AnySource.as_str().to_string(),
            0,
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting usdc price"))?;
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
    let usdc_apr_usdc = querier
        .get_incentive_apr(
            EarnType::UsdcProgram as i32,
            ElysDenom::Usdc.as_str().to_string(),
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting usdc apr in usdc"))?;
    let eden_apr_usdc = querier
        .get_incentive_apr(
            EarnType::UsdcProgram as i32,
            ElysDenom::Eden.as_str().to_string(),
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting eden apr in usdc"))?;

    let usdc_apr_edenb = querier
        .get_incentive_apr(
            EarnType::EdenBProgram as i32,
            ElysDenom::Usdc.as_str().to_string(),
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting usdc apr in edenb"))?;
    let eden_apr_edenb = querier
        .get_incentive_apr(
            EarnType::EdenBProgram as i32,
            ElysDenom::Eden.as_str().to_string(),
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting eden apr in edenb"))?;

    let usdc_apr_eden = querier
        .get_incentive_apr(
            EarnType::EdenProgram as i32,
            ElysDenom::Usdc.as_str().to_string(),
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting usdc apr in eden"))?;
    let eden_apr_eden = querier
        .get_incentive_apr(
            EarnType::EdenProgram as i32,
            ElysDenom::Eden.as_str().to_string(),
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting eden apr in eden"))?;
    let edenb_apr_eden = querier
        .get_incentive_apr(
            EarnType::EdenProgram as i32,
            ElysDenom::EdenBoost.as_str().to_string(),
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting edenb apr in eden"))?;

    let usdc_apr_elys = querier
        .get_incentive_apr(
            EarnType::ElysProgram as i32,
            ElysDenom::Usdc.as_str().to_string(),
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting usdc apr in elys"))?;
    let eden_apr_elys = querier
        .get_incentive_apr(
            EarnType::ElysProgram as i32,
            ElysDenom::Eden.as_str().to_string(),
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting eden apr in elys"))?;
    let edenb_apr_elys = querier
        .get_incentive_apr(
            EarnType::ElysProgram as i32,
            ElysDenom::EdenBoost.as_str().to_string(),
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting edenb apr in elys"))?;

    for address in resp.addresses {
        let mut history: HashMap<String, AccountSnapshot> =
            if let Some(histories) = HISTORY.may_load(deps.storage, &address)? {
                update_history(histories, &env.block, &expiration)
            } else {
                HashMap::new()
            };
        let account_balances = deps.querier.query_all_balances(&address)?;
        let order_balances = get_all_orders(&deps.querier, &trade_shield_address, &address)?;
        let staked_response = get_staked_assets(
            &deps,
            &address,
            uusdc_usd_price,
            uelys_price_in_uusdc,
            usdc_denom.to_owned(),
            usdc_base_denom.to_owned(),
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
        let perpetual_response = match get_perpetuals(
            deps.as_ref(),
            trade_shield_address.clone(),
            &usdc_denom,
            address.clone(),
        ) {
            Ok(perpetual_response) => perpetual_response,
            Err(_) => PerpetualAssets {
                total_perpetual_asset_balance: DecCoin::new(Decimal256::zero(), &usdc_denom),
                perpetual_asset: vec![],
            },
        };

        let new_part = create_new_part(
            &env.block,
            &querier,
            &expiration,
            account_balances,
            order_balances,
            staked_response,
            rewards_response,
            perpetual_response,
            &usdc_denom,
        )?;

        let today = get_today(&env.block);

        if let Some(part) = new_part {
            history.insert(today, part);
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
    usdc_denom: &String,
) -> StdResult<Option<AccountSnapshot>> {
    let date = match expiration {
        Expiration::AtHeight(_) => Expiration::AtHeight(block.height),
        Expiration::AtTime(_) => Expiration::AtTime(block.time),
        Expiration::Never {} => panic!("never expire"),
    };

    let available_asset_balance: Vec<CoinValue> = account_balances
        .iter()
        .filter_map(
            |coin| match CoinValue::from_coin(coin, querier, usdc_denom) {
                Ok(res) => Some(res),
                Err(_) => None,
            },
        )
        .collect();

    let in_orders_asset_balance: Vec<CoinValue> = orders_balances
        .iter()
        .filter_map(
            |coin| match CoinValue::from_coin(coin, querier, usdc_denom) {
                Ok(res) => Some(res),
                Err(_) => None,
            },
        )
        .collect();

    let mut total_available_balance = DecCoin::new(Decimal256::zero(), usdc_denom);
    let mut total_in_orders_balance = DecCoin::new(Decimal256::zero(), usdc_denom);

    for balance in &available_asset_balance {
        total_available_balance.amount = total_available_balance
            .amount
            .checked_add(Decimal256::from(balance.amount_usdc.clone()))?
    }

    for balance in &in_orders_asset_balance {
        total_in_orders_balance.amount = total_in_orders_balance
            .amount
            .checked_add(Decimal256::from(balance.amount_usdc.clone()))?
    }

    let mut total_value_per_asset: HashMap<&String, CoinValue> = HashMap::new();

    for available in available_asset_balance.iter() {
        total_value_per_asset
            .entry(&available.denom)
            .and_modify(|e| {
                e.amount_token += available.amount_token.clone();
                e.amount_usdc += available.amount_usdc.clone();
            })
            .or_insert_with(|| available.clone());
    }

    for in_order in in_orders_asset_balance.iter() {
        total_value_per_asset
            .entry(&in_order.denom)
            .and_modify(|e| {
                e.amount_token += in_order.amount_token.clone();
                e.amount_usdc += in_order.amount_usdc.clone();
            })
            .or_insert_with(|| in_order.clone());
    }

    let total_value_per_asset: Vec<CoinValue> = total_value_per_asset.values().cloned().collect();

    let total_liquid_asset_balance = DecCoin::new(
        Decimal256::from(
            total_value_per_asset
                .iter()
                .map(|v| v.amount_usdc)
                .fold(Decimal::zero(), |acc, item| acc + item),
        ),
        usdc_denom,
    );

    let reward = rewards_response.rewards;
    let portfolio_usd = DecCoin::new(
        total_liquid_asset_balance
            .amount
            .checked_add(Decimal256::from(
                staked_assets_resp.total_staked_balance.amount.clone(),
            ))?
            .checked_add(
                perpetual_response
                    .total_perpetual_asset_balance
                    .amount
                    .clone(),
            )?,
        usdc_denom,
    );
    let reward_usd: DecCoin = DecCoin::new(Decimal256::from(reward.clone().total_usd), usdc_denom);
    let total_balance = DecCoin::new(
        portfolio_usd.amount.checked_add(reward_usd.amount)?,
        usdc_denom,
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
                usdc_denom,
            ),
            liquidity_positions_usd: DecCoin::new(Decimal256::zero(), usdc_denom),
            leverage_lp_usd: DecCoin::new(Decimal256::zero(), usdc_denom),
            perpetual_assets_usd: perpetual_response.total_perpetual_asset_balance.clone(),
            usdc_earn_usd: DecCoin::new(Decimal256::zero(), usdc_denom),
            borrows_usd: DecCoin::new(Decimal256::zero(), usdc_denom),
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
    history: HashMap<String, AccountSnapshot>,
    block_info: &BlockInfo,
    expiration: &Expiration,
) -> HashMap<String, AccountSnapshot> {
    let mut history = history;

    let expiration = match expiration {
        Expiration::AtHeight(h) => Timestamp::from_seconds(h * 3), // since a block is created every 3 seconds
        Expiration::AtTime(t) => t.clone(),
        _ => panic!("never expire"),
    };

    if expiration > block_info.time {
        return history;
    }

    let expired_date = match get_raw_today(&block_info)
        .checked_sub_days(Days::new(expiration.seconds() / (24 * 3600)))
    {
        Some(date) => date.format("%Y-%m-%d").to_string(),
        None => panic!("invalid date"),
    };

    if history.get(&expired_date).is_some() {
        history.remove_entry(&expired_date);
        history.remove(&expired_date);
    };

    return history;
}

#[cfg(test)]
mod tests {
    use crate::types::{Reward, StakedAssets};

    use super::*;
    use cosmwasm_std::Timestamp;

    #[test]
    fn test_update_history() {
        let mut history: HashMap<String, AccountSnapshot> = HashMap::new();

        let snapshot = AccountSnapshot {
            date: Expiration::AtTime(Timestamp::from_seconds(1707306681)),
            total_balance: TotalBalance {
                total_balance: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                portfolio_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                reward_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
            },
            portfolio: Portfolio {
                balance_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                liquid_assets_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                staked_committed_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                liquidity_positions_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                leverage_lp_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                perpetual_assets_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                usdc_earn_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                borrows_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
            },
            reward: Reward {
                usdc_usd: Decimal::zero(),
                eden_usd: Decimal::zero(),
                eden_boost: Uint128::zero(),
                other_usd: Decimal::zero(),
                total_usd: Decimal::zero(),
            },
            liquid_asset: LiquidAsset {
                total_liquid_asset_balance: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                total_available_balance: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                total_in_orders_balance: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                available_asset_balance: vec![],
                in_orders_asset_balance: vec![],
                total_value_per_asset: vec![],
            },
            staked_assets: StakedAssets::default(),
            perpetual_assets: PerpetualAssets {
                total_perpetual_asset_balance: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                perpetual_asset: vec![],
            },
        };

        let old_snapshot = AccountSnapshot {
            date: Expiration::AtTime(Timestamp::from_seconds(1706701881)),
            total_balance: TotalBalance {
                total_balance: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                portfolio_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                reward_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
            },
            portfolio: Portfolio {
                balance_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                liquid_assets_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                staked_committed_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                liquidity_positions_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                leverage_lp_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                perpetual_assets_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                usdc_earn_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                borrows_usd: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
            },
            reward: Reward {
                usdc_usd: Decimal::zero(),
                eden_usd: Decimal::zero(),
                eden_boost: Uint128::zero(),
                other_usd: Decimal::zero(),
                total_usd: Decimal::zero(),
            },
            liquid_asset: LiquidAsset {
                total_liquid_asset_balance: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                total_available_balance: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                total_in_orders_balance: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                available_asset_balance: vec![],
                in_orders_asset_balance: vec![],
                total_value_per_asset: vec![],
            },
            staked_assets: StakedAssets::default(),
            perpetual_assets: PerpetualAssets {
                total_perpetual_asset_balance: DecCoin::new(Decimal256::zero(), "usdc".to_string()),
                perpetual_asset: vec![],
            },
        };

        let block_info = BlockInfo {
            height: 0,
            time: Timestamp::from_seconds(1707306681),
            chain_id: "chain_id".to_string(),
        };
        let expiration = Expiration::AtTime(Timestamp::from_seconds(24 * 3600 * 7));

        history.insert("2024-02-07".to_string(), snapshot.clone());
        history.insert("2024-01-31".to_string(), old_snapshot.clone());

        assert!(history.get("2024-02-07").is_some());
        assert!(history.get("2024-01-31").is_some());

        let history = update_history(history, &block_info, &expiration);

        assert!(history.get("2024-02-07").is_some());
        assert!(history.get("2024-01-31").is_none());
    }
}
