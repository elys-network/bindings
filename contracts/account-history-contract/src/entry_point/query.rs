use crate::{
    action::query::{
        get_eden_boost_earn_program_details, get_eden_earn_program_details,
        get_elys_earn_program_details, get_estaking_rewards, get_liquid_assets,
        get_masterchef_pending_rewards, get_masterchef_pool_apr, get_masterchef_stable_stake_apr,
        get_membership_tier, get_perpetuals_assets, get_pool_balances, get_rewards,
        get_staked_assets, get_usdc_earn_program_details,
    },
    states::USER_ADDRESS_QUEUE,
    types::AccountSnapshotGenerator,
};

#[cfg(feature = "debug")]
use crate::action::query::{
    all, exit_pool_estimation, get_pools, join_pool_estimation, last_snapshot, params,
    pool_asset_estimation, user_snapshots, user_value,
};

use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, Env, StdResult};
use cw2::CONTRACT;
use elys_bindings::{
    account_history::types::ElysDenom, query_resp::QueryAprResponse, ElysQuerier, ElysQuery,
};

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
        GetRewards { user_address } => to_json_binary(&get_rewards(deps, user_address, env)?),

        GetMembershipTier { user_address } => {
            to_json_binary(&get_membership_tier(env, deps, user_address)?)
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

        GetEstakingRewards { address } => to_json_binary(&get_estaking_rewards(deps, address)?),

        GetMasterchefParams {} => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.masterchef_params()?)
        }

        GetMasterchefPoolInfo { pool_id } => {
            let querier = ElysQuerier::new(&deps.querier);
            to_json_binary(&querier.masterchef_pool_info(pool_id)?)
        }

        GetMasterchefPendingRewards { address } => {
            to_json_binary(&get_masterchef_pending_rewards(deps, address)?)
        }

        GetMasterChefPoolApr { pool_ids } => {
            to_json_binary(&get_masterchef_pool_apr(deps, pool_ids)?)
        }

        GetMasterchefStableStakeApr { denom } => {
            to_json_binary(&get_masterchef_stable_stake_apr(deps, denom)?)
        }

        // debug only
        #[cfg(feature = "debug")]
        Params {} => to_json_binary(&params(deps)?),
        #[cfg(feature = "debug")]
        All { pagination } => to_json_binary(&all(deps, pagination)?),
        #[cfg(feature = "debug")]
        UserSnapshots { user_address } => to_json_binary(&user_snapshots(env, deps, user_address)?),
        #[cfg(feature = "debug")]
        LastSnapshot { user_address } => to_json_binary(&last_snapshot(deps, user_address, env)?),
        #[cfg(feature = "debug")]
        UserValue { user_address } => to_json_binary(&user_value(env, deps, user_address)?),
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
        #[cfg(feature = "debug")]
        GetEdenEarnProgramDetails { address } => {
            let querier = ElysQuerier::new(&deps.querier);
            let aprs = querier.get_incentive_aprs().unwrap_or_default();

            let generator = AccountSnapshotGenerator::new(&deps)?;
            let program = get_eden_earn_program_details(
                &deps,
                Some(address.to_owned()),
                ElysDenom::Eden.as_str().to_string(),
                generator.metadata.uusdc_usd_price,
                generator.metadata.uelys_price_in_uusdc,
                QueryAprResponse {
                    apr: aprs.usdc_apr_eden,
                },
                QueryAprResponse {
                    apr: aprs.eden_apr_eden,
                },
                QueryAprResponse {
                    apr: aprs.edenb_apr_eden,
                },
            )
            .unwrap_or_default();
            to_json_binary(&program)
        }
        #[cfg(feature = "debug")]
        GetEdenBoostEarnProgramDetails { address } => {
            let querier = ElysQuerier::new(&deps.querier);
            let aprs = querier.get_incentive_aprs().unwrap_or_default();
            let program = get_eden_boost_earn_program_details(
                &deps,
                Some(address.to_owned()),
                ElysDenom::EdenBoost.as_str().to_string(),
                QueryAprResponse {
                    apr: aprs.usdc_apr_edenb,
                },
                QueryAprResponse {
                    apr: aprs.eden_apr_edenb,
                },
            )
            .unwrap_or_default();

            to_json_binary(&program)
        }
        #[cfg(feature = "debug")]
        GetElysEarnProgramDetails { address } => {
            let querier = ElysQuerier::new(&deps.querier);
            let aprs = querier.get_incentive_aprs().unwrap_or_default();

            let generator = AccountSnapshotGenerator::new(&deps)?;
            let program = get_elys_earn_program_details(
                &deps,
                Some(address.to_owned()),
                ElysDenom::Elys.as_str().to_string(),
                generator.metadata.uusdc_usd_price,
                generator.metadata.uelys_price_in_uusdc,
                QueryAprResponse {
                    apr: aprs.usdc_apr_elys,
                },
                QueryAprResponse {
                    apr: aprs.eden_apr_elys,
                },
                QueryAprResponse {
                    apr: aprs.edenb_apr_elys,
                },
            )
            .unwrap_or_default();

            to_json_binary(&program)
        }
        #[cfg(feature = "debug")]
        GetUsdcEarnProgramDetails { address } => {
            let generator = AccountSnapshotGenerator::new(&deps)?;
            let program = get_usdc_earn_program_details(
                &deps,
                Some(address.to_owned()),
                generator.metadata.usdc_denom.to_owned(),
                generator.metadata.usdc_base_denom.to_owned(),
                generator.metadata.uusdc_usd_price,
            )
            .unwrap_or_default();

            to_json_binary(&program)
        }
        #[cfg(feature = "debug")]
        IncentiveAprs { .. } => {
            let querier = ElysQuerier::new(&deps.querier);
            let response = querier.get_incentive_aprs().unwrap_or_default();

            to_json_binary(&response)
        }
        AddressQueueSize {} => {
            let user_address_queue_data_size = USER_ADDRESS_QUEUE.len(deps.storage)? as u128;

            to_json_binary(&user_address_queue_data_size)
        }
        Version {} => to_json_binary(&CONTRACT.load(deps.storage)?),
    }
}
