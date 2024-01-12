use crate::trade_shield::types::MarginOrder;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetMarginOrderResp {
    pub order: MarginOrder,
}
