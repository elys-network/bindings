use super::*;
use cw2::CONTRACT;
use elys_bindings::ElysQuery;
use msg::QueryMsg;

pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query::earn;
    use QueryMsg::*;

    match msg {
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
        Version {} => Ok(to_json_binary(&CONTRACT.load(deps.storage)?)?),
    }
}
