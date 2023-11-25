use super::*;
use crate::action::query::user_value;
use msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        UserValue { user_address } => to_json_binary(&user_value(deps, env.block, user_address)?),
        Accounts {} => to_json_binary(&{
            let querrier = ElysQuerier::new(&deps.querier);

            let resp = querrier.accounts(types::PageRequest::new(8))?;
            resp
        }),
    }
}
