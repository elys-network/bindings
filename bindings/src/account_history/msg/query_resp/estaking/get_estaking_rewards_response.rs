use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

use crate::account_history::types::CoinValue;

#[cw_serde]
pub struct GetEstakingRewardsResponse {
    pub rewards: Vec<(String, CoinValue)>,
    pub total: Vec<Coin>,
}

#[cw_serde]
pub struct DelegationDelegatorReward {
    pub validator_address: String,
    pub reward: Vec<CoinValue>,
}
