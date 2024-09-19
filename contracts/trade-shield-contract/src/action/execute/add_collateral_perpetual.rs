use super::*;
use cosmwasm_std::StdError;

pub fn perpetual_add_collateral(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    env: Env,
    id: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    let collateral = cw_utils::one_coin(&info)?;
    if PERPETUAL_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("perpetual endpoint are disable").into());
    }
    let msg = ElysMsg::perpetual_add_collateral(
        env.contract.address.as_str(),
        id,
        collateral.amount.into(),
        info.sender.as_str(),
    );

    let resp = Response::new().add_message(CosmosMsg::Custom(msg));

    Ok(resp)
}
