use crate::bindings::query::ElysQuery;

use super::*;
use msg::QueryMsg;

pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query::earn;
    use action::query::pod;
    use QueryMsg::*;

    match msg {
        // Pod dashboard
        GetPodRewards { user_address } => {
            Ok(to_json_binary(&pod::get_pod_rewards(deps, user_address)?)?)
        }
        GetPodLiquidityPositions {} => {
            Ok(to_json_binary(&pod::get_pod_liquidity_positions(deps)?)?)
        }
        GetPodLiquidityPosition { pool_id } => Ok(to_json_binary(
            &pod::get_pod_liquidity_position(deps, pool_id)?,
        )?),

        // Earn Program
        GetEdenBoostEarnProgramDetails { user_address } => Ok(to_json_binary(
            &earn::get_eden_boost_earn_program_details(deps, user_address)?,
        )?),
        GetEdenEarnProgramDetails { user_address } => Ok(to_json_binary(
            &earn::get_eden_earn_program_details(deps, user_address)?,
        )?),
        GetElysEarnProgramDetails { user_address } => Ok(to_json_binary(
            &earn::get_elys_earn_program_details(deps, user_address)?,
        )?),
        GetAllValidators { delegator_addr } => Ok(to_json_binary(&earn::get_all_validators(
            deps,
            delegator_addr,
        )?)?),
        GetDelegatorValidators { delegator_addr } => Ok(to_json_binary(
            &earn::get_delegator_validators(deps, delegator_addr)?,
        )?),
        GetUsdcEarnProgramDetails { user_address } => Ok(to_json_binary(
            &earn::get_usdc_earn_program_details(deps, user_address)?,
        )?),
        GetDelegations { delegator_addr } => Ok(to_json_binary(&earn::get_delegations(
            deps,
            delegator_addr,
        )?)?),
        GetUnbondingDelegations { delegator_addr } => Ok(to_json_binary(
            &earn::get_unbonding_delegations(deps, delegator_addr)?,
        )?),
        GetCommitments { delegator_addr } => Ok(to_json_binary(&earn::get_commitments(
            deps,
            delegator_addr,
        )?)?),

        // Liquidity Pools
        GetLiquidityPools {
            pool_ids,
            filter_type,
            pagination,
        } => Ok(to_json_binary(&earn::get_pools(
            deps,
            pool_ids,
            filter_type,
            pagination,
        )?)?),

        // Specific function for querying USDC oracle price
        GetUsdcPrice {} => Ok(to_json_binary(&earn::get_usdc_price(deps)?)?),
    }
}
