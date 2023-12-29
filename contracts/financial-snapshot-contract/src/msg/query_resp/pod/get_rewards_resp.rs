use crate::types::Reward;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetRewardsResp {
    pub rewards: Reward,
}
