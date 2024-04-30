use crate::action::query::{
    get_liquid_assets, get_membership_tier, get_perpetuals_assets, get_pool_balances,
    get_portfolio, get_rewards, get_staked_assets, get_total_balance,
};

#[cfg(feature = "debug")]
use crate::action::query::{
    all, exit_pool_estimation, get_pools, get_pools_apr, join_pool_estimation, last_snapshot,
    params, pool_asset_estimation, user_snapshots, user_value,
};

use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, Env, StdResult};
use elys_bindings::{ElysQuerier, ElysQuery};

use crate::msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Accounts { pagination } => to_json_binary(&{
            let querrier = ElysQuerier::new(&deps.querier);

            let resp = querrier.accounts(pagination)?;
            resp
        }),

        GetLiquidAssets { user_address } => {
            to_json_binary(&get_liquid_assets(deps, user_address, env)?)
        }
        GetStakedAssets { user_address } => {
            to_json_binary(&get_staked_assets(deps, user_address, env)?)
        }
        GetPoolBalances { user_address } => {
            to_json_binary(&get_pool_balances(deps, user_address, env)?)
        }
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
        GetAssetPrice { asset } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_asset_price(asset)?)
        }

        GetLiquidityPools {
            pool_ids,
            filter_type,
            pagination,
        } => to_json_binary(&get_pools(deps, pool_ids, filter_type, pagination)?),

        GetLiquidityPoolsApr { pool_ids } => to_json_binary(&get_pools_apr(deps, pool_ids)?),

        JoinPoolEstimation {
            pool_id,
            amounts_in,
        } => to_json_binary(&join_pool_estimation(deps, pool_id, amounts_in)?),

        PoolAssetEstimation { pool_id, amount } => {
            to_json_binary(&pool_asset_estimation(deps, pool_id, amount)?)
        }

        ExitPoolEstimation {
            pool_id,
            exit_fiat_amount,
        } => to_json_binary(&exit_pool_estimation(deps, pool_id, exit_fiat_amount)?),

        GetAssetPriceFromDenomInToDenomOut {
            denom_in,
            denom_out,
        } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(
                &querier.get_asset_price_from_denom_in_to_denom_out(denom_in, denom_out)?,
            )
        }

        // debug only
        #[cfg(feature = "debug")]
        Params {} => to_json_binary(&params(deps)?),
        #[cfg(feature = "debug")]
        All { pagination } => to_json_binary(&all(deps, pagination)?),
        #[cfg(feature = "debug")]
        UserSnapshots { user_address } => to_json_binary(&user_snapshots(deps, user_address)?),
        #[cfg(feature = "debug")]
        LastSnapshot { user_address } => to_json_binary(&last_snapshot(deps, user_address, env)?),
        #[cfg(feature = "debug")]
        UserValue { user_address } => to_json_binary(&user_value(deps, user_address)?),
        #[cfg(feature = "debug")]
        CommitmentStakedPositions { delegator_address } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_staked_positions(delegator_address)?)
        }
        #[cfg(feature = "debug")]
        CommitmentUnStakedPositions { delegator_address } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_unstaked_positions(delegator_address)?)
        }
        #[cfg(feature = "debug")]
        CommitmentRewardsSubBucketBalanceOfDenom {
            address,
            denom,
            program,
        } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_sub_bucket_rewards_balance(address, denom, program)?)
        }
        #[cfg(feature = "debug")]
        CommitmentStakedBalanceOfDenom { address, denom } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_staked_balance(address, denom)?)
        }
        #[cfg(feature = "debug")]
        StableStakeBalanceOfBorrow {} => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_borrowed_balance()?)
        }
        #[cfg(feature = "debug")]
        StableStakeParams {} => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_stable_stake_params()?)
        }
        #[cfg(feature = "debug")]
        CommitmentVestingInfo { address } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_vesting_info(address)?)
        }
        #[cfg(feature = "debug")]
        Balance { address, denom } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_balance(address, denom)?)
        }
        #[cfg(feature = "debug")]
        AmmPriceByDenom { token_in, discount } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.get_amm_price_by_denom(token_in, discount)?)
        }
    }
}
