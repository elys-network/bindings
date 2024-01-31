use crate::trade_shield::types::PerpetualPositionPlus;
use crate::types::PageResponse;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetPerpetualPositionsForAddressResp {
    pub mtps: Vec<PerpetualPositionPlus>,
    pub pagination: PageResponse,
}
