use std::collections::HashMap;

use cosmwasm_schema::cw_serde;

use crate::account_history::types::DecCoinValue;

#[cw_serde]
#[derive(Default)]
pub struct GetMasterchefUserPendingRewardResponse {
    pub rewards: HashMap<u64, DecCoinValue>,
    pub total_rewards: Vec<DecCoinValue>,
}
