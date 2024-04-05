use cosmwasm_schema::cw_serde;

use crate::trade_shield::types::Status;

#[cw_serde]
pub struct GetSortedOrderListResp {
    pub orders_states: Vec<OrdersStates>,
}

#[cw_serde]
pub struct OrdersStates {
    pub id: u64,
    pub status: Status,
    pub is_in_pending: bool,
    pub found: bool,
}
