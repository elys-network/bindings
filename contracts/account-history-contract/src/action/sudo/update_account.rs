use std::collections::HashMap;

use cosmwasm_std::{BlockInfo, DepsMut, Env, Response, StdResult, Timestamp};
use cw_utils::Expiration;

use crate::{
    states::{HISTORY, METADATA, PROCESSED_ACCOUNT_PER_BLOCK, USER_ADDRESS_QUEUE},
    types::AccountSnapshotGenerator,
    utils::get_today,
};
use elys_bindings::{
    account_history::types::PortfolioBalanceSnapshot, ElysMsg, ElysQuerier, ElysQuery,
};

pub fn update_account(deps: DepsMut<ElysQuery>, env: Env) -> StdResult<Response<ElysMsg>> {
    let querier = ElysQuerier::new(&deps.querier);

    // update metadata prices
    let mut metadata = METADATA.load(deps.storage)?;
    metadata = metadata.update_prices(&querier)?;
    METADATA.save(deps.storage, &metadata)?;

    let today = get_today(&env.block);
    let user_address_queue: Vec<String> = USER_ADDRESS_QUEUE
        .prefix_range(deps.storage, None, None, cosmwasm_std::Order::Descending)
        .filter_map(|res| res.ok().map(|(addr, _)| addr))
        .collect();

    let processed_account_per_block: usize =
        PROCESSED_ACCOUNT_PER_BLOCK.load(deps.storage)? as usize;
    let processed_account_per_block = if processed_account_per_block > user_address_queue.len() {
        user_address_queue.len()
    } else {
        processed_account_per_block
    };

    let mut histories: Vec<(String, Option<HashMap<String, PortfolioBalanceSnapshot>>)> = vec![];
    for i in 0..processed_account_per_block {
        USER_ADDRESS_QUEUE.remove(deps.storage, &user_address_queue[i]);
        if let Some(history) = HISTORY.may_load(deps.storage, &user_address_queue[i])? {
            if history.get(&today.clone()).is_some() {
                // skip if the account has been updated today
                continue;
            } else {
                histories.push((user_address_queue[i].clone(), Some(history)));
            }
        } else {
            histories.push((user_address_queue[i].clone(), None));
        }
    }

    let generator = AccountSnapshotGenerator::new(&deps.as_ref())?;

    for (address, history) in histories.iter_mut() {
        let history_data = history.get_or_insert(HashMap::new());

        clean_up_history(history_data, &env.block, &generator.expiration);

        let new_part = generator.generate_portfolio_balance_snapshot_for_address(
            &querier,
            &deps.as_ref(),
            &env,
            address,
        )?;

        history_data.insert(today.clone(), new_part);
        HISTORY.save(deps.storage, &address, &history_data)?;
    }

    Ok(Response::default())
}

fn clean_up_history(
    history: &mut HashMap<String, PortfolioBalanceSnapshot>,
    block_info: &BlockInfo,
    expiration: &Expiration,
) {
    if history.is_empty() {
        return;
    }
    let expiration = match expiration {
        Expiration::AtHeight(h) => Timestamp::from_seconds(h * 3), // since a block is created every 3 seconds
        Expiration::AtTime(t) => t.clone(),
        _ => panic!("never expire"),
    };

    if expiration > block_info.time {
        return;
    }

    let expired_date = block_info.time.minus_seconds(expiration.seconds());
    let history_vec: Vec<(String, PortfolioBalanceSnapshot)> =
        history.clone().into_iter().collect();

    for (date, snapshot) in history_vec {
        let timestamp = match snapshot.date {
            Expiration::AtHeight(h) => Timestamp::from_seconds(h * 3),
            Expiration::AtTime(t) => t.clone(),
            _ => panic!("never expire"),
        };
        if timestamp <= expired_date {
            history.remove(&date);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Decimal256, Timestamp};

    #[test]
    fn test_clean_up_history() {
        let mut history: HashMap<String, PortfolioBalanceSnapshot> = HashMap::new();

        let snapshot = PortfolioBalanceSnapshot {
            date: Expiration::AtTime(Timestamp::from_seconds(1707306681)),
            total_balance_usd: Decimal256::zero(),
            portfolio_balance_usd: Decimal256::zero(),
        };

        let old_snapshot = PortfolioBalanceSnapshot {
            date: Expiration::AtTime(Timestamp::from_seconds(1706701881)),
            total_balance_usd: Decimal256::zero(),
            portfolio_balance_usd: Decimal256::zero(),
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

        clean_up_history(&mut history, &block_info, &expiration);

        assert!(history.get("2024-02-07").is_some());
        assert!(history.get("2024-01-31").is_none());
    }
}
