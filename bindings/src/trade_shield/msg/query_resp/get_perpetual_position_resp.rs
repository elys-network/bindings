use crate::trade_shield::types::PerpetualPositionPlus;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetPerpetualPositionResp {
    pub mtp: PerpetualPositionPlus,
}
