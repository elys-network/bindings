use crate::{
    msg::query_resp::GetTotalBalanceResp,
    states::{HISTORY, VALUE_DENOM},
    types::TotalBalance,
};
use cosmwasm_std::{DecCoin, Decimal256, Deps, StdResult};
use elys_bindings::ElysQuery;

pub fn get_pod_total_balance(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<GetTotalBalanceResp> {
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let snapshots = HISTORY.load(deps.storage, &user_address)?;
    let snapshot = match snapshots.last().cloned() {
        Some(expr) => expr,
        None => {
            return Ok(GetTotalBalanceResp {
                balances: TotalBalance {
                    total_balance: DecCoin::new(Decimal256::zero(), &value_denom),
                    portfolio_usd: DecCoin::new(Decimal256::zero(), &value_denom),
                    reward_usd: DecCoin::new(Decimal256::zero(), &value_denom),
                },
            })
        }
    };
    let portfolio_usd = DecCoin::new(
        snapshot
            .total_liquid_asset_balance
            .amount
            .checked_add(Decimal256::from(snapshot.total_staked_asset_balance))?,
        &value_denom,
    );
    let resp = GetTotalBalanceResp {
        balances: TotalBalance {
            total_balance: portfolio_usd.clone(),
            portfolio_usd: portfolio_usd.clone(),
            reward_usd: DecCoin::new(Decimal256::zero(), &value_denom),
        },
    };
    Ok(resp)
}
