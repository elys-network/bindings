use crate::trade_shield::types::MarginPositionPlus;
use cosmwasm_schema::cw_serde;
use crate::types::PageResponse;

#[cw_serde]
pub struct GetMarginPositionsResp {
    pub mtps: Vec<MarginPositionPlus>,
    pub pagination: PageResponse,
}
