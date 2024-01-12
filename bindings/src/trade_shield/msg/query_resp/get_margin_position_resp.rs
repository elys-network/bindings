use crate::trade_shield::types::MarginPositionPlus;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetMarginPositionResp {
    pub mtp: MarginPositionPlus,
}
