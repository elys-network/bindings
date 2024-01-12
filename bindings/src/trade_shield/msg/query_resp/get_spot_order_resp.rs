use crate::trade_shield::types::SpotOrder;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetSpotOrderResp {
    pub order: SpotOrder,
}
