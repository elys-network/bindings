use cosmwasm_std::StdError;

use crate::helper::remove_spot_order;

use super::*;

pub fn cancel_spot_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_id: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    if SWAP_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("swap is disable").into());
    }
    let order: SpotOrder = match SPOT_ORDER.may_load(deps.storage, order_id)? {
        Some(order) => order,
        None => return Err(ContractError::OrderNotFound { order_id }),
    };

    if order.owner_address != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    if order.status != Status::Pending {
        return Err(ContractError::CancelStatusError {
            order_id,
            status: order.status,
        });
    }

    let refund_msg = BankMsg::Send {
        to_address: order.owner_address.to_string(),
        amount: vec![order.order_amount.clone()],
    };

    let resp = Response::new()
        .add_message(CosmosMsg::Bank(refund_msg))
        .add_event(Event::new("cancel_spot_order").add_attribute("order_id", order_id.to_string()));

    remove_spot_order(order.order_id, Status::Canceled, deps.storage)?;

    let number_of_pending_order = NUMBER_OF_PENDING_ORDER.load(deps.storage)? - 1;
    NUMBER_OF_PENDING_ORDER.save(deps.storage, &number_of_pending_order)?;

    Ok(resp)
}
