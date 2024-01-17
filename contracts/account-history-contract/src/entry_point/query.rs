use super::*;
use crate::{
    action::query::{get_staked_assets, get_total_value_per_asset, params, user_value},
    states::HISTORY,
    types::AccountSnapshot,
};
use cosmwasm_std::{Order, StdError};
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
        All {} => to_json_binary(&{
            let list = HISTORY
                .prefix_range(deps.storage, None, None, Order::Ascending)
                .filter_map(|res| res.ok())
                .collect::<Vec<(String, Vec<AccountSnapshot>)>>();
            list
        }),
        LastSnapshot { user_address } => to_json_binary(&{
            let snapshots = HISTORY.load(deps.storage, &user_address)?;
            match snapshots.last().cloned() {
                Some(expr) => expr,
                None => return Err(StdError::not_found("account snapshot")),
            }
        }),
        GetLiquidAssets { user_address } => {
            to_json_binary(&get_total_value_per_asset(deps, user_address)?)
        }
        GetStakedAssets { user_address } => to_json_binary(&get_staked_assets(deps, user_address)?),
        Params {} => to_json_binary(&params(deps)?),
    }
}
