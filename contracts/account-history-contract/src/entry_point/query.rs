use super::*;
use crate::{
    action::query::{
        get_liquid_assets, get_membership_tier, get_perpetuals_assets, get_portfolio, get_rewards,
        get_staked_assets, get_total_balance, params, user_value,
    },
    states::HISTORY,
    types::AccountSnapshot,
};
use cosmwasm_std::{Order, StdError};
use msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        UserValue { user_address } => to_json_binary(&user_value(deps, env.block, user_address)?),
        Accounts { pagination } => to_json_binary(&{
            let querrier = ElysQuerier::new(&deps.querier);

            let resp = querrier.accounts(pagination)?;
            resp
        }),
        All {} => to_json_binary(&{
            let list = HISTORY
                .prefix_range(deps.storage, None, None, Order::Ascending)
                .filter_map(|res| res.ok())
                .collect::<Vec<(String, Vec<AccountSnapshot>)>>();
            list
        }),
        UserSnapshots { user_address } => {
            to_json_binary(&{ HISTORY.load(deps.storage, &user_address)? })
        }
        LastSnapshot { user_address } => to_json_binary(&{
            let snapshots = HISTORY.load(deps.storage, &user_address)?;
            match snapshots.last().cloned() {
                Some(expr) => expr,
                None => return Err(StdError::not_found("account snapshot")),
            }
        }),
        GetLiquidAssets { user_address } => to_json_binary(&get_liquid_assets(deps, user_address)?),
        GetStakedAssets { user_address } => to_json_binary(&get_staked_assets(deps, user_address)?),
        Params {} => to_json_binary(&params(deps)?),
        GetPortfolio { user_address } => to_json_binary(&get_portfolio(deps, user_address)?),
        GetTotalBalance { user_address } => to_json_binary(&get_total_balance(deps, user_address)?),
        GetRewards { user_address } => to_json_binary(&get_rewards(deps, user_address)?),
        GetMembershipTier { user_address } => {
            to_json_binary(&get_membership_tier(deps, env.block, user_address)?)
        }
        GetPerpetualAssets { user_address } => {
            to_json_binary(&get_perpetuals_assets(deps, user_address)?)
        }

        // debug only
        CommitmentStakedPositions { delegator_address } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_staked_positions(delegator_address)?)
        }
        CommitmentUnStakedPositions { delegator_address } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_unstaked_positions(delegator_address)?)
        }
        CommitmentRewardsSubBucketBalanceOfDenom {
            address,
            denom,
            program,
        } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_sub_bucket_rewards_balance(address, denom, program)?)
        }
        CommitmentStakedBalanceOfDenom { address, denom } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_staked_balance(address, denom)?)
        }
        StableStakeBalanceOfBorrow {} => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_borrowed_balance()?)
        }
        CommitmentVestingInfo { address } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_vesting_info(address)?)
        }
        Balance { address, denom } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_balance(address, denom)?)
        }
        AmmPriceByDenom { token_in, discount } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_amm_price_by_denom(token_in, discount)?)
        }
    }
}
