use crate::types::Rewards;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct UserRewardsResponse {
    pub rewards: Rewards,
}
