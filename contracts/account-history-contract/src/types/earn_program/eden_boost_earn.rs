use crate::types::{BalanceReward, AprUsdc};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct EdenBoostEarnProgram {
    // should be 0 initially. In days
    pub bonding_period: u64,
    // The APR For the Eden Boost Earn Program.
    pub apr: AprUsdc,
    // available should be the user Eden Boost liquid balance on Elys and returned
    // only if address is included in the request object.
    pub available: Option<Uint128>,
    // it should return how much Eden Boost the user has staked in this program ONLY.
    // it should only be included if address is in the request object.
    pub staked: Option<Uint128>,
    // The rewards the user currently has on the Eden Boost Earn Program.
    // It should be in the response only if the address is in the request object.
    // rewards are either USDC or EDEN.
    pub rewards: Option<Vec<BalanceReward>>,
}
