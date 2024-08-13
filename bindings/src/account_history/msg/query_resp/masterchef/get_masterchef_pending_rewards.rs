use std::collections::HashMap;

use cosmwasm_schema::cw_serde;

use crate::trade_shield::types::CoinValue;

#[cw_serde]
#[derive(Default)]
pub struct GetMasterchefUserPendingRewardResponse {
    pub rewards: HashMap<u64, Vec<CoinValue>>,
    pub total_rewards: Vec<CoinValue>,
}
