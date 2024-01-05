use cosmwasm_std::{coin, BlockInfo, Coin, Decimal, Uint128};
use cw_utils::Expiration;
use elys_bindings::query_resp::AmmSwapEstimationByDenomResponse;

use crate::types::AccountValue;

use super::*;

pub fn update_account(deps: DepsMut<ElysQuery>, env: Env) -> StdResult<Response<ElysMsg>> {
    let querier = ElysQuerier::new(&deps.querier);
    let value_denom = VALUE_DENOM.load(deps.storage)?;

    let mut pagination = PAGINATION.load(deps.storage)?;
    let expiration = EXPIRATION.load(deps.storage)?;

    let resp = querier.accounts(Some(pagination.clone()))?;

    pagination.update(resp.pagination.next_key);
    PAGINATION.save(deps.storage, &pagination)?;

    for address in resp.addresses {
        let mut history = if let Some(history) = HISTORY.may_load(deps.storage, &address)? {
            update_history(history, &env.block, &expiration)
        } else {
            vec![]
        };
        let account_balences = deps.querier.query_all_balances(&address)?;
        let new_part: AccountValue = create_new_part(
            &env.block,
            &querier,
            &expiration,
            account_balences,
            &value_denom,
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
    account_balences: Vec<Coin>,
    value_denom: &String,
) -> StdResult<AccountValue> {
    let date = match expiration {
        Expiration::AtHeight(_) => Expiration::AtHeight(block.height),
        Expiration::AtTime(_) => Expiration::AtTime(block.time),
        Expiration::Never {} => panic!("never expire"),
    };

    let mut value = Uint128::zero();

    for balence in account_balences {
        if &balence.denom == value_denom {
            value += balence.amount;
            continue;
        }
        let AmmSwapEstimationByDenomResponse { amount, .. } = querier
            .amm_swap_estimation_by_denom(
                &balence,
                &balence.denom,
                value_denom,
                &Decimal::zero(),
            )?;
        value += amount.amount;
    }

    Ok(AccountValue {
        date,
        account_value: coin(value.u128(), value_denom),
    })
}

fn update_history(
    history: Vec<AccountValue>,
    block_info: &BlockInfo,
    expiration: &Expiration,
) -> Vec<AccountValue> {
    let clean_history: Vec<AccountValue> = history
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
