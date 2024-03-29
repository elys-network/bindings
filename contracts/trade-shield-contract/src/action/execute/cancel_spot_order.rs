use cosmwasm_std::StdError;

use super::*;

pub fn cancel_spot_order(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    order_id: u64,
) -> Result<Response<ElysMsg>, ContractError> {
    if SWAP_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("swap is disable").into());
    }
    let mut order: SpotOrder = match SPOT_ORDER.may_load(deps.storage, order_id)? {
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

    order.status = Status::Canceled;

    let refund_msg = BankMsg::Send {
        to_address: order.owner_address.to_string(),
        amount: vec![order.order_amount.clone()],
    };

    let resp = Response::new()
        .add_message(CosmosMsg::Bank(refund_msg))
        .add_event(Event::new("cancel_spot_order").add_attribute("order_id", order_id.to_string()));

    SPOT_ORDER.save(deps.storage, order_id, &order)?;
    PENDING_SPOT_ORDER.remove(deps.storage, order_id);
    let key = order.gen_key()?;
    let mut vec = SORTED_PENDING_SPOT_ORDER.load(deps.storage, key.as_str())?;
    let index = vec.binary_search(&order.order_id).map_err(|_| StdError::not_found("order id not found"))?;
    vec.remove(index);
    SORTED_PENDING_SPOT_ORDER.save(deps.storage, key.as_str(), &vec)?;

    Ok(resp)
}
