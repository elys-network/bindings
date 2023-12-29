use crate::types::MarginPositionPlus;
use cosmwasm_schema::cw_serde;
use elys_bindings::types::PageResponse;

#[cw_serde]
pub struct GetMarginPositionsForAddressResp {
    pub mtps: Vec<MarginPositionPlus>,
    pub pagination: PageResponse,
}
