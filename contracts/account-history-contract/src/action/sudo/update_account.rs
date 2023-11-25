use cosmwasm_std::{coin, BlockInfo, Coin, Uint128};
use cw_utils::Expiration;
use elys_bindings::query_resp::{AmmSwapEstimationByDenomResponse, AuthAccountsResponse};

use crate::types::AccountValue;

use super::*;

pub fn update_account(deps: DepsMut<ElysQuery>, env: Env) -> StdResult<Response<ElysMsg>> {
    let querier = ElysQuerier::new(&deps.querier);

    let mut pagination = PAGINATION.load(deps.storage)?;
    let expiration = EXPIRATION.load(deps.storage)?;

    let AuthAccountsResponse {
        accounts,
        pagination: pagination_resp,
    }: AuthAccountsResponse = querier.accounts(pagination.clone())?;

    pagination.update(pagination_resp.next_key);
    PAGINATION.save(deps.storage, &pagination)?;

    for account in accounts {
        let mut history = if let Some(history) = HISTORY.may_load(deps.storage, &account.address)? {
            update_history(history, &env.block, &expiration)
        } else {
            vec![]
        };
        let account_balences = deps.querier.query_all_balances(&account.address)?;
        let new_part: AccountValue =
            create_new_part(&env.block, &querier, &expiration, account_balences)?;
        history.push(new_part);
        HISTORY.save(deps.storage, &account.address, &history)?;
    }

    Ok(Response::default())
}

fn create_new_part(
    block: &BlockInfo,
    querier: &ElysQuerier<'_>,
    expiration: &Expiration,
    account_balences: Vec<Coin>,
) -> StdResult<AccountValue> {
    let date = match expiration {
        Expiration::AtHeight(_) => Expiration::AtHeight(block.height),
        Expiration::AtTime(_) => Expiration::AtTime(block.time),
        Expiration::Never {} => panic!("never expire"),
    };

    let mut value = Uint128::zero();

    for balence in account_balences {
        if balence.denom == "uusdc" {
            value += balence.amount;
            continue;
        }
        let AmmSwapEstimationByDenomResponse { amount, .. } =
            querier.amm_swap_estimation_by_denom(&balence, &balence.denom, "uusdc")?;
        value += amount.amount;
    }

    Ok(AccountValue {
        date,
        account_value: coin(value.u128(), "uusdc"),
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
