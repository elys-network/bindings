use std::collections::HashMap;

use cosmwasm_std::{BlockInfo, DepsMut, Env, Response, StdError, StdResult, Timestamp};
use cw_utils::Expiration;

use crate::{
    states::{HISTORY, METADATA, PAGINATION},
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

    // update pagination
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

    let mut histories: Vec<(String, Option<HashMap<String, PortfolioBalanceSnapshot>>)> = vec![];
    for address in resp.addresses {
        if let Some(history) = HISTORY.may_load(deps.storage, &address)? {
            if history.get(&today.clone()).is_some() {
                // skip if the account has been updated today
                continue;
            } else {
                histories.push((address, Some(history)));
            }
        } else {
            histories.push((address, None));
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
