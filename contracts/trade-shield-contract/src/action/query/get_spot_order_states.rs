use cosmwasm_std::{Deps, StdResult};
use elys_bindings::{
    trade_shield::{
        msg::query_resp::GetSpotOrderStatesResp,
        states::{PENDING_SPOT_ORDER, SORTED_PENDING_SPOT_ORDER, SPOT_ORDER},
        types::SpotOrderType,
    },
    ElysQuery,
};

pub fn get_spot_order_states(
    deps: Deps<ElysQuery>,
    order_id: u64,
) -> StdResult<GetSpotOrderStatesResp> {
    let order = SPOT_ORDER.load(deps.storage, order_id)?;
    let is_in_pending = PENDING_SPOT_ORDER
        .may_load(deps.storage, order_id)?
        .is_some();

    let is_in_pending_sorted_array = match order.order_type {
        SpotOrderType::MarketBuy => false,
        _ => {
            let sorted_id_list =
                SORTED_PENDING_SPOT_ORDER.may_load(deps.storage, order.gen_key()?.as_str())?;

            match sorted_id_list {
                Some(list) => list.iter().find(|&id| *id == order_id).is_some(),
                None => false,
            }
        }
    };

    Ok(GetSpotOrderStatesResp {
        order,
        is_in_pending,
        is_in_pending_sorted_array,
    })
}
