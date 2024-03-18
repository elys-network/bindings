use crate::trade_shield::types::PerpetualOrderPlus;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetPerpetualOrderResp {
    pub order: PerpetualOrderPlus,
}
