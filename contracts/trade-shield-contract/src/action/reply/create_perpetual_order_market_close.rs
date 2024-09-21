use super::*;
use cosmwasm_std::{from_json, Binary, StdError, SubMsgResult};

pub fn reply_to_create_perpetual_market_close(
    deps: DepsMut<ElysQuery>,
    data: Option<Binary>,
    module_resp: SubMsgResult,
) -> Result<Response<ElysMsg>, ContractError> {
    let resp_data = match module_resp.into_result() {
        Ok(resp) => resp.data,
        Err(err) => return Err(StdError::generic_err(err).into()),
    };

    let order_id: u64 = match data {
        Some(order_id) => from_json(&order_id)?,
        None => return Err(StdError::generic_err("no meta_data").into()),
    };

    if resp_data.is_none() {
        return Err(StdError::generic_err("no data from response").into());
    }

    let mut order: PerpetualOrderV2 = match PERPETUAL_ORDER_V2.may_load(deps.storage, order_id)? {
        Some(order) => order,
        None => return Err(ContractError::OrderNotFound { order_id }),
    };

    let perpetual_resp: PerpetualCloseResponse = match from_json(&resp_data.unwrap()) {
        Ok(resp) => resp,
        Err(err) => return Err(err.into()),
    };

    order.status = Status::Executed;

    let resp = Response::new().add_event(
        Event::new("reply_to_create_perpetual_market_close")
            .add_attribute("perpetual_order_id", order_id.to_string())
            .add_attribute(
                "perpetual_trading_position_closed_id",
                perpetual_resp.id.to_string(),
            )
            .add_attribute(
                "perpetual_amount_closed",
                perpetual_resp.amount.i128().to_string(),
            ),
    );

    PERPETUAL_ORDER_V2.save(deps.storage, order_id, &order)?;

    Ok(resp)
}
