use cosmwasm_schema::cw_serde;

use crate::query_resp::PoolApr;

#[cw_serde]
pub struct MasterChefPoolAprResponse {
    pub data: Vec<PoolApr>,
}