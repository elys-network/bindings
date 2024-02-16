use super::*;
use elys_bindings::ElysQuery;
use msg::QueryMsg;

pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query::earn;
    use action::query::pod;
    use QueryMsg::*;

    match msg {
        // Pod dashboard
        GetPodLiquidityPositions {} => {
            Ok(to_json_binary(&pod::get_pod_liquidity_positions(deps)?)?)
        }
        GetPodLiquidityPosition { pool_id } => Ok(to_json_binary(
            &pod::get_pod_liquidity_position(deps, pool_id)?,
        )?),

        // Earn Program
        GetAllValidators { delegator_addr } => Ok(to_json_binary(&earn::get_all_validators(
            deps,
            delegator_addr,
        )?)?),
        GetDelegatorValidators { delegator_addr } => Ok(to_json_binary(
            &earn::get_delegator_validators(deps, delegator_addr)?,
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

        // Specific function for querying USDC oracle price
        GetUsdcPrice {} => Ok(to_json_binary(&earn::get_usdc_price(deps)?)?),
    }
}
