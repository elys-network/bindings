use cosmwasm_std::{Int128, StdError};

use super::*;

pub fn close_perpetual_position(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    env: Env,
    id: u64,
    amount: Int128,
) -> Result<Response<ElysMsg>, ContractError> {
    if PERPETUAL_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("perpetual endpoint are disable").into());
    }
    let msg = ElysMsg::perpetual_close_position(
        env.contract.address.as_str(),
        id,
        amount.i128(),
        info.sender.as_str(),
    );

    let resp = Response::new().add_message(CosmosMsg::Custom(msg));

    Ok(resp)
}
