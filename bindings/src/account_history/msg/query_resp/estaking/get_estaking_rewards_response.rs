use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

use crate::trade_shield::types::CoinValue;

#[cw_serde]
pub struct GetEstakingRewardsResponse {
    pub rewards: Vec<CoinValue>,
    pub total: Vec<Coin>,
}

#[cw_serde]
pub struct DelegationDelegatorReward {
    pub validator_address: String,
    pub reward: Vec<CoinValue>,
}
