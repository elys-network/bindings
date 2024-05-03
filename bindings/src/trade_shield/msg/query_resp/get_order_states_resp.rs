use cosmwasm_schema::cw_serde;

use crate::trade_shield::types::SpotOrder;

#[cw_serde]
pub struct GetSpotOrderStatesResp {
    pub order: SpotOrder,
    pub is_in_pending: bool,
    pub is_in_pending_sorted_array: bool,
}
