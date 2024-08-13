use crate::account_history::types::{CoinValue, Reward};

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetRewardsResp {
    pub rewards_map: Reward,
    pub rewards: Vec<CoinValue>,
}
