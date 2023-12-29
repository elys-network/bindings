use crate::types::{BalanceAvailable, VestingDetail, BalanceReward, AprElys};
use crate::bindings::query_resp::StakedAvailable;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct EdenEarnProgram {
    // should be 0 initially. In days
    pub bonding_period: u64,
    // The APR For the EDEN Earn Program.
    pub apr: AprElys,
    // available should be the user EDEN liquid balance on Elys and returned
    // only if address is included in the request object.
    pub available: Option<BalanceAvailable>,
    // it should return how much EDEN the user has staked in this program ONLY.
    // it should only be included if address is in the request object.
    pub staked: Option<StakedAvailable>,
    // The rewards the user currently has on the EDEN Earn Program.
    // It should be in the response only if the address is in the request object.
    // rewards are either USDC, EDEN or EDEN Boost.
    // Eden Boost doesnt have USD amount.
    pub rewards: Option<Vec<BalanceReward>>,
    // The sum of all the total_vest.
    pub vesting: Option<BalanceAvailable>,
    // A list of all the vesting details for the EDEN program.
    // it should only be included if address is in the request object.
    pub vesting_details: Option<Vec<VestingDetail>>,
}