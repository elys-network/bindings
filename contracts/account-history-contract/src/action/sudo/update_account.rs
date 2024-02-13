use std::collections::HashMap;

use chrono::Days;
use cosmwasm_std::{BlockInfo, DepsMut, Env, Response, StdError, StdResult, Timestamp};
use cw_utils::Expiration;

use crate::{
    states::{HISTORY, PAGINATION},
    types::AccountSnapshotGenerator,
    utils::{get_raw_today, get_today},
};
use elys_bindings::{account_history::types::AccountSnapshot, ElysMsg, ElysQuerier, ElysQuery};

pub fn update_account(deps: DepsMut<ElysQuery>, env: Env) -> StdResult<Response<ElysMsg>> {
    let querier = ElysQuerier::new(&deps.querier);

    let mut pagination = PAGINATION.load(deps.storage)?;

    let resp = querier.accounts(Some(pagination.clone())).map_err(|e| {
        StdError::generic_err(format!(
            "failed to get accounts with pagination {:?}: {}",
            pagination, e
        ))
    })?;

    pagination.update(resp.pagination.next_key);
    PAGINATION.save(deps.storage, &pagination)?;

    let today = get_today(&env.block);

    let mut addresses_to_process: Vec<String> = vec![];
    for address in resp.addresses {
        if let Some(history) = HISTORY.may_load(deps.storage, &address)? {
            if history.get(&today.clone()).is_some() {
                // skip if the account has been updated today
                continue;
            }
        }
        addresses_to_process.push(address)
    }

    // Read common variables before looping
    // To enhance querying speed.
    let generator = AccountSnapshotGenerator::new(&deps.as_ref())?;

    for address in addresses_to_process.iter() {
        let mut history: HashMap<String, AccountSnapshot> =
            if let Some(histories) = HISTORY.may_load(deps.storage, &address)? {
                update_history(histories, &env.block, &generator.expiration)
            } else {
                HashMap::new()
            };

        let new_part = generator.generate_account_snapshot_for_address(
            &querier,
            &deps.as_ref(),
            &env,
            address,
        )?;

        if let Some(part) = new_part {
            history.insert(today.clone(), part);
        }
        if history.is_empty() {
            HISTORY.remove(deps.storage, &address);
        } else {
            HISTORY.save(deps.storage, &address, &history)?;
        }
    }

    Ok(Response::default())
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
    use super::*;
    use cosmwasm_std::{DecCoin, Decimal, Decimal256, Timestamp, Uint128};
    use elys_bindings::account_history::types::{
        LiquidAsset, PerpetualAssets, Portfolio, Reward, StakedAssets, TotalBalance,
    };

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
