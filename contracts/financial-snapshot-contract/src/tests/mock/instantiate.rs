use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{bindings::query::ElysQuery, states::*};

#[cw_serde]
pub struct InstantiateMockMsg {
    pub epoch_cycle_interval: u128,
}

pub fn instantiate(
    _deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMockMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}
