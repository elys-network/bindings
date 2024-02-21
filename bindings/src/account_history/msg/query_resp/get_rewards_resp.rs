use crate::account_history::types::{Reward, BalanceReward};

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetRewardsResp {
    pub rewards_map: Reward,
    pub rewards: Vec<BalanceReward>
}
