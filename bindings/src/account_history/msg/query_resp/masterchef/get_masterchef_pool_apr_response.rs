use cosmwasm_schema::cw_serde;

use crate::account_history::types::DecCoinValue;


#[cw_serde]
pub struct MasterChefPoolAprResponse {
    pub data: Vec<PoolAprValue>,
}

#[cw_serde]
#[derive(Default)]
pub struct PoolAprValue{
    pub pool_id: u64,
    pub eden_apr: DecCoinValue,
    pub usdc_apr: DecCoinValue,
    pub total_apr: DecCoinValue,
}

