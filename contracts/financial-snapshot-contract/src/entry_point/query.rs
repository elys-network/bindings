use crate::bindings::query::ElysQuery;

use super::*;
use msg::QueryMsg;

pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query::pod;
    use action::query::earn;
    use QueryMsg::*;

    match msg {
        // Pod dashboard
        GetPodPortfolio { address } => Ok(to_json_binary(&pod::get_pod_portfolio(deps, address)?)?),
        GetPodTotalBalance { address } => Ok(to_json_binary(&pod::get_pod_total_balance(deps, address)?)?),
        GetPodRewards { address } => Ok(to_json_binary(&pod::get_pod_rewards(deps, address)?)?),
        GetPodLiquidAssets { } => Ok(to_json_binary(&pod::get_pod_liquid_assets(deps)?)?),
        GetPodLiquidAsset { asset } => Ok(to_json_binary(&pod::get_pod_liquid_asset(deps, asset)?)?),
        GetPodLiquidityPositions { } => Ok(to_json_binary(&pod::get_pod_liquidity_positions(deps)?)?),
        GetPodLiquidityPosition { pool_id } => Ok(to_json_binary(&pod::get_pod_liquidity_position(deps, pool_id)?)?),

        // Earn Program
        GetEdenBoostEarnProgramDetails { address, asset } => Ok(to_json_binary(&earn::get_eden_boost_earn_program_details(deps, address, asset)?)?),
        GetEdenEarnProgramDetails { address, asset } => Ok(to_json_binary(&earn::get_eden_earn_program_details(deps, address, asset)?)?),
        GetElysEarnProgramDetails { address, asset } => Ok(to_json_binary(&earn::get_elys_earn_program_details(deps, address, asset)?)?),
        GetAllValidators { delegator_addr } => Ok(to_json_binary(&earn::get_all_validators(deps, delegator_addr)?)?),
        GetDelegatorValidators { delegator_addr } => Ok(to_json_binary(&earn::get_delegator_validators(deps, delegator_addr)?)?),
        GetUsdcEarnProgramDetails { address, asset } => Ok(to_json_binary(&earn::get_usdc_earn_program_details(deps, address, asset)?)?),
        GetDelegations { delegator_addr } => Ok(to_json_binary(&earn::get_delegations(deps, delegator_addr)?)?),
        GetUnbondingDelegations { delegator_addr } => Ok(to_json_binary(&earn::get_unbonding_delegations(deps, delegator_addr)?)?),
        GetCommitments { delegator_addr } => Ok(to_json_binary(&earn::get_commitments(deps, delegator_addr)?)?),

        // Liquidity Pools
        GetLiquidityPools { pool_ids, filter_type, pagination} => Ok(to_json_binary(&earn::get_pools(deps, pool_ids, filter_type, pagination)?)?),

        // Specific function for querying USDC oracle price
        GetUsdcPrice { } => Ok(to_json_binary(&earn::get_usdc_price(deps)?)?),
    }
}
