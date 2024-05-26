use std::collections::HashMap;

use chrono::NaiveDateTime;
use cosmwasm_std::{BlockInfo, DepsMut, Env, Response, StdError, StdResult, Storage, Timestamp};
use cw_utils::Expiration;

use crate::{
    states::{
        HISTORY, METADATA, PROCESSED_ACCOUNT_PER_BLOCK, UPDATE_ACCOUNT_ENABLED, USER_ADDRESS_QUEUE,
    },
    types::AccountSnapshotGenerator,
    utils::get_today,
};
use elys_bindings::{ElysMsg, ElysQuerier, ElysQuery};

pub fn update_account(
    deps: DepsMut<ElysQuery>,
    env: Env,
    save_update_account_enabled_param: Option<bool>,
) -> StdResult<Response<ElysMsg>> {
    if UPDATE_ACCOUNT_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("Update account is disabled"));
    }
    let querier = ElysQuerier::new(&deps.querier);

    // update metadata prices
    let mut metadata = METADATA.load(deps.storage)?;
    metadata = metadata.update_prices(&querier)?;
    METADATA.save(deps.storage, &metadata)?;

    let today = get_today(&env.block);
    let mut user_address_queue: Vec<String> = USER_ADDRESS_QUEUE
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

    let mut today_snapshots = match HISTORY.may_load(deps.storage, &today)? {
        Some(snapshots) => snapshots,
        None => HashMap::new(),
    };

    let generator = AccountSnapshotGenerator::new(&deps.as_ref())?;

    for _ in 0..processed_account_per_block {
        if user_address_queue.is_empty() == true {
            break;
        }

        // remove the first element from the queue
        let user_address = user_address_queue.remove(0);
        // remove the user address from the queue
        USER_ADDRESS_QUEUE.remove(deps.storage, &user_address);

        if today_snapshots.get(&user_address).is_some() {
            // skip if the account has been updated today
            continue;
        }

        let new_part = generator.generate_portfolio_balance_snapshot_for_address(
            &querier,
            &deps.as_ref(),
            &env,
            &user_address,
        )?;
        today_snapshots.insert(user_address.clone(), new_part);
    }

    HISTORY.save(deps.storage, &today, &today_snapshots)?;

    clean_up_history(deps.storage, &env.block, &generator.expiration);

    if let Some(save_update_account_enabled_param) = save_update_account_enabled_param {
        UPDATE_ACCOUNT_ENABLED.save(deps.storage, &save_update_account_enabled_param)?;
    }

    Ok(Response::default())
}

fn clean_up_history(storage: &mut dyn Storage, block_info: &BlockInfo, expiration: &Expiration) {
    let expiration = match expiration {
        Expiration::AtHeight(h) => Timestamp::from_seconds(h * 3), // since a block is created every 3 seconds
        Expiration::AtTime(t) => t.clone(),
        _ => panic!("never expire"),
    };

    if expiration > block_info.time {
        return;
    }

    let expired_date = NaiveDateTime::from_timestamp_opt(
        block_info
            .time
            .minus_seconds(expiration.seconds())
            .seconds() as i64,
        0,
    )
    .expect("Failed to convert block time to date")
    .format("%Y-%m-%d")
    .to_string();

    HISTORY.remove(storage, expired_date.as_str());
}
