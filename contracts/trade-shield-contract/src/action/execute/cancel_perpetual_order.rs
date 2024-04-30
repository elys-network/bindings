use cosmwasm_std::StdError;

use crate::helper::remove_perpetual_order;

use super::*;

pub fn cancel_perpetual_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_id: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    if PERPETUAL_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("perpetual endpoint are disable").into());
    }
    let order = match PERPETUAL_ORDER.may_load(deps.storage, order_id)? {
        Some(order) => order,
        None => return Err(ContractError::OrderNotFound { order_id }),
    };

    let order_type = order.order_type.clone();

    if order.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    if order.status != Status::Pending {
        return Err(ContractError::CancelStatusError {
            order_id,
            status: order.status.clone(),
        });
    }

    let refund_msg = BankMsg::Send {
        to_address: order.owner.clone(),
        amount: vec![order.collateral.clone()],
    };

    let resp = Response::new().add_event(
        Event::new("cancel_perpetual_order")
            .add_attribute("perpetual_order_id", order.order_id.to_string()),
    );

    remove_perpetual_order(order_id, Status::Canceled, deps.storage)?;

    let number_of_pending_order = NUMBER_OF_PENDING_ORDER.load(deps.storage)? - 1;
    NUMBER_OF_PENDING_ORDER.save(deps.storage, &number_of_pending_order)?;

    if order_type == PerpetualOrderType::LimitOpen {
        Ok(resp.add_message(CosmosMsg::Bank(refund_msg)))
    } else {
        Ok(resp)
    }
}
