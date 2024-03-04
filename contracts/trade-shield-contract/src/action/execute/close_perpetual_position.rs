use cosmwasm_std::Int128;

use super::*;

pub fn close_perpetual_position(
    info: MessageInfo,
    env: Env,
    id: u64,
    amount: Int128,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::perpetual_close_position(
        env.contract.address.as_str(),
        id,
        amount.i128(),
        info.sender.as_str(),
    );

    let resp = Response::new().add_message(CosmosMsg::Custom(msg));

    Ok(resp)
}
