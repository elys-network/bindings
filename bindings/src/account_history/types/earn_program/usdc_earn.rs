use crate::{
    account_history::types::{AprUsdc, CoinValue},
    query_resp::{BalanceBorrowed, StakedAvailable},
    types::BalanceAvailable,
};

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct UsdcEarnProgram {
    // should be 0 initially. In days
    pub bonding_period: u64,
    // The APR For the USDC Earn Program.
    pub apr: AprUsdc,
    // available should be the user USDC liquid balance on Elys and returned
    // only if address is included in the request object.
    pub available: Option<BalanceAvailable>,
    // it should return how much USDC the user has staked in this program ONLY.
    // it should only be included if address is in the request object.
    pub staked: Option<StakedAvailable>,
    // The rewards the user currently has on the USDC Earn Program.
    // It should be in the response only if the address is in the request object.
    // rewards are either USDC or EDEN.
    pub rewards: Option<Vec<CoinValue>>,
    // The amount that has been borrowed from the user staked positions.
    pub borrowed: Option<BalanceBorrowed>,
}

// implement default
impl Default for UsdcEarnProgram {
    fn default() -> Self {
        Self {
            bonding_period: 0,
            apr: AprUsdc::default(),
            available: None,
            staked: None,
            rewards: None,
            borrowed: None,
        }
    }
}
