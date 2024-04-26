use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, StdResult};

use crate::{account_history::types::DecCoinValue, query_resp::PoolAprValue, ElysQuerier};

#[cw_serde]
pub struct QueryPoolAprsResponse {
    pub data: Vec<PoolApr>,
}

#[cw_serde]
pub struct PoolApr {
    pub pool_id: u64,
    pub eden_apr: DecCoin,
    pub usdc_apr: DecCoin,
    pub total_apr: DecCoin,
}

impl PoolApr {
    pub fn to_dec_coin_value(
        &self,
        querier: &ElysQuerier<'_>,
        usdc_denom: &String,
    ) -> StdResult<PoolAprValue> {
        Ok(PoolAprValue {
            pool_id: self.pool_id,
            eden_apr: DecCoinValue::from_dec_coin(&self.eden_apr, querier, usdc_denom)?,
            usdc_apr: DecCoinValue::from_dec_coin(&self.usdc_apr, querier, usdc_denom)?,
            total_apr: DecCoinValue::from_dec_coin(&self.total_apr, querier, usdc_denom)?,
        })
    }
}
