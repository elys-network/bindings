use super::*;
use crate::action::query::*;
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
        UserRewards { user_address } => Ok(to_json_binary(&user_rewards(deps, user_address)?)?),
    }
}
