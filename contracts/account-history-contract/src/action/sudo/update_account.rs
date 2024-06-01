use chrono::NaiveDateTime;
use cosmwasm_std::{DepsMut, Env, Response, StdResult, Timestamp};
use cw_utils::Expiration;

use crate::{
    states::{HISTORY, METADATA, OLD_HISTORY_2, PROCESSED_ACCOUNT_PER_BLOCK, USER_ADDRESS_QUEUE},
    types::AccountSnapshotGenerator,
    utils::get_today,
};
use elys_bindings::{ElysMsg, ElysQuerier, ElysQuery};

pub fn update_account(deps: DepsMut<ElysQuery>, env: Env) -> StdResult<Response<ElysMsg>> {
    let querier = ElysQuerier::new(&deps.querier);

    // update metadata prices
    let mut metadata = METADATA.load(deps.storage)?;
    metadata = metadata.update_prices(&querier)?;
    METADATA.save(deps.storage, &metadata)?;

    let today = get_today(&env.block);

    let processed_account_per_block: usize =
        PROCESSED_ACCOUNT_PER_BLOCK.load(deps.storage)? as usize;

    let generator = AccountSnapshotGenerator::new(&deps.as_ref())?;

    for _ in 0..processed_account_per_block {
        if USER_ADDRESS_QUEUE.is_empty(deps.storage)? == true {
            break;
        }

        // remove the first element from the queue
        let user_address = if let Some(addr) = USER_ADDRESS_QUEUE.pop_back(deps.storage)? {
            addr.to_string()
        } else {
            break;
        };

        let key = today.clone() + &user_address;

        if let Some(_) = HISTORY.may_load(deps.storage, &key)? {
            // skip if the account has been updated today
            continue;
        }

        let new_part = generator.generate_portfolio_balance_snapshot_for_address(
            &querier,
            &deps.as_ref(),
            &env,
            &user_address,
        )?;
        HISTORY.save(deps.storage, &key, &new_part)?;
    }

    Ok(Response::default())
}

pub fn clean_up_history(
    deps: &mut DepsMut<ElysQuery>,
    env: Env,
    limit: u64,
) -> StdResult<Response<ElysMsg>> {
    let generator = AccountSnapshotGenerator::new(&deps.as_ref())?;
    let block_info = env.block;
    let expiration = match generator.expiration {
        Expiration::AtHeight(h) => Timestamp::from_seconds(h * 3), // since a block is created every 3 seconds
        Expiration::AtTime(t) => t.clone(),
        _ => panic!("never expire"),
    };

    if expiration > block_info.time {
        return Ok(Response::default());
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

    // Delete limit values
    for _ in 0..limit {
        if let Some(val) = HISTORY.first(deps.storage)? {
            let date_part = &val.0[0..10];
            if date_part < expired_date.as_str() {
                HISTORY.remove(deps.storage, &val.0);
            }
        } else {
            break;
        }
    }
    Ok(Response::default())
}

pub fn clean_old_history(
    deps: &mut DepsMut<ElysQuery>,
    limit: u64,
) -> StdResult<Response<ElysMsg>> {
    // Delete limit values
    for _ in 0..limit {
        if let Some(val) = OLD_HISTORY_2.first(deps.storage)? {
            OLD_HISTORY_2.remove(deps.storage, &val.0);
        } else {
            break;
        }
    }
    Ok(Response::default())
}
