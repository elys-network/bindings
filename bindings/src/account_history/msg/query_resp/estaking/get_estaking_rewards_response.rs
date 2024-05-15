use cosmwasm_schema::cw_serde;
use cosmwasm_std::DecCoin;

use crate::account_history::types::Coin256Value;

#[cw_serde]
pub struct GetEstakingRewardsResponse {
    pub rewards: Vec<(String, Coin256Value)>,
    pub total: Vec<DecCoin>,
}

#[cw_serde]
pub struct DelegationDelegatorReward {
    pub validator_address: String,
    pub reward: Vec<Coin256Value>,
}
