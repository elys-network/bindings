use crate::types::LiquidityPosition;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetLiquidityPositionsResp {
    pub data: Vec<LiquidityPosition>,
}
