use crate::types::{BalanceAvailable, BalanceReward, AprElys, StakedPosition, UnstakedPosition};
use crate::bindings::query_resp::StakedAvailable;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct ElysEarnProgram {
    // should be 0 initially. In days
    pub bonding_period: u64,
    // The APR For the Elys Earn Program.
    pub apr: AprElys,
    // available should be the user Elys liquid balance on Elys and returned
    // only if address is included in the request object.
    pub available: Option<BalanceAvailable>,
    // it should return how much Elys the user has staked in this program ONLY.
    // it should only be included if address is in the request object.
    pub staked: Option<StakedAvailable>,
    // The rewards the user currently has on the Elys Earn Program.
    // It should be in the response only if the address is in the request object.
    // rewards are either USDC, EDEN or EDEN Boost.
    pub rewards: Option<Vec<BalanceReward>>,
    // All the positions the user has staked on the ELYS program.
    // It should be in the response only if the address is in the request object.
    pub staked_positions: Option<Vec<StakedPosition>>,
    // The positions the user has decided to unstake.
    // It should be in the response only if the address is in the request object.
    pub unstaked_positions:Option<Vec<UnstakedPosition>>,
}
