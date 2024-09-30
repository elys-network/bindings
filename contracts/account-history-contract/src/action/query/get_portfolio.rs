use crate::{msg::query_resp::GetPortfolioResp, types::AccountSnapshotGenerator};
use cosmwasm_std::{Deps, Env, SignedDecimal256, StdResult};
use elys_bindings::{ElysQuerier, ElysQuery};

pub fn get_portfolio(
    deps: Deps<ElysQuery>,
    user_address: String,
    env: Env,
) -> StdResult<GetPortfolioResp> {
    unimplemented!()
}
