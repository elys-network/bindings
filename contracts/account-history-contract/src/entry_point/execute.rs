use super::*;
use crate::{action::sudo::update_account, msg::ExecuteMsg, states::PAGINATION};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ElysQuery>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<ElysMsg>> {
    use ExecuteMsg::*;

    match msg {
        SetLimit { limit } => {
            let mut p = PAGINATION.load(deps.storage)?;
            p.limit = limit;
            PAGINATION.save(deps.storage, &p)?;
            Ok(Response::new())
        }
        UpdateAcount {} => update_account(deps, env),
    }
}
