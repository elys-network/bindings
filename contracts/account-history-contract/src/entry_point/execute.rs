use super::*;
use crate::{
    msg::ExecuteMsg,
    states::{EXPIRATION, PAGINATION},
};

pub fn execute(
    deps: DepsMut<ElysQuery>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<ElysMsg>> {
    use action::execute::*;
    use ExecuteMsg::*;

    match msg {
        SetLimit { limit } => {
            let mut p = PAGINATION.load(deps.storage)?;
            p.limit = limit;
            PAGINATION.save(deps.storage, &p)?;
            Ok(Response::new())
        }
        SetExpiration { expiration } => {
            EXPIRATION.save(deps.storage, &expiration)?;
            Ok(Response::new())
        }
        UpdateAccounts {} => update_account(deps, env),
    }
}
