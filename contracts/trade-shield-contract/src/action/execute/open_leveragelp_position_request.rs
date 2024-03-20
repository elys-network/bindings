use super::*;
use cosmwasm_std::{Int128, SignedDecimal, StdError};

pub fn open_leveragelp_position_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    amm_pool_id: u64,
    collateral_asset: String,
    collateral_amount: Int128,
    leverage: SignedDecimal,
    stop_loss_price: SignedDecimal,
) -> Result<Response<ElysMsg>, ContractError> {
    if LEVERAGE_ENABLE.load(deps.storage)? == false {
        return Err(StdError::generic_err("leverage endpoint are disable").into());
    }

    let msg: ElysMsg = ElysMsg::leveragelp_open_position(
        info.sender.into_string(),
        amm_pool_id,
        collateral_asset,
        collateral_amount,
        leverage,
        stop_loss_price,
    );

    let resp = Response::new().add_message(msg);
    Ok(resp)
}
