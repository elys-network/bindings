use crate::action::query::{
    all, get_liquid_assets, get_membership_tier, get_perpetuals_assets, get_portfolio, get_rewards,
    get_staked_assets, get_total_balance, last_snapshot, params, user_snapshots, user_value,
};

use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, Env, StdResult};
use elys_bindings::{ElysQuerier, ElysQuery};

use crate::msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        UserValue { user_address } => to_json_binary(&user_value(deps, user_address)?),
        Accounts { pagination } => to_json_binary(&{
            let querrier = ElysQuerier::new(&deps.querier);

            let resp = querrier.accounts(pagination)?;
            resp
        }),

        All {} => to_json_binary(&all(deps)?),

        UserSnapshots { user_address } => to_json_binary(&user_snapshots(deps, user_address)?),

        LastSnapshot { user_address } => to_json_binary(&last_snapshot(deps, user_address, env)?),
        GetLiquidAssets { user_address } => {
            to_json_binary(&get_liquid_assets(deps, user_address, env)?)
        }
        GetStakedAssets { user_address } => {
            to_json_binary(&get_staked_assets(deps, user_address, env)?)
        }
        Params {} => to_json_binary(&params(deps)?),
        GetPortfolio { user_address } => to_json_binary(&get_portfolio(deps, user_address, env)?),
        GetTotalBalance { user_address } => {
            to_json_binary(&get_total_balance(deps, env, user_address)?)
        }
        GetRewards { user_address } => to_json_binary(&get_rewards(deps, user_address, env)?),

        GetMembershipTier { user_address } => {
            to_json_binary(&get_membership_tier(deps, user_address)?)
        }
        GetPerpetualAssets { user_address } => {
            to_json_binary(&get_perpetuals_assets(deps, user_address, env)?)
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
        StableStakeParams {} => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_stable_stake_params()?)
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
