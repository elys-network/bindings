use cosmwasm_std::{DepsMut, Env, Response, StdResult};

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

pub fn update_account_chain(deps: DepsMut<ElysQuery>, env: Env) -> StdResult<Response<ElysMsg>> {
    let processed_account_per_block: usize =
        PROCESSED_ACCOUNT_PER_BLOCK.load(deps.storage)? as usize;

    let mut msgs = vec![];
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

        let msg: ElysMsg =
            ElysMsg::tier_set_portfolio(env.contract.address.to_string(), user_address);
        msgs.push(msg)
    }

    Ok(Response::default().add_messages(msgs))
}
