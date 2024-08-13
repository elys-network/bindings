use crate::{
    account_history::types::{AprElys, CoinValue, ElysDenom},
    query_resp::StakedAvailable,
    trade_shield::types::BalanceAvailable,
    types::VestingDetail,
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Uint128};

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
    pub rewards: Option<Vec<CoinValue>>,
    // The sum of all the total_vest.
    pub vesting: BalanceAvailable,
    // A list of all the vesting details for the EDEN program.
    // it should only be included if address is in the request object.
    pub vesting_details: Option<Vec<VestingDetail>>,
}

impl EdenEarnProgram {
    fn to_coin(&self, amount: Uint128) -> Coin {
        Coin::new(u128::from(amount), ElysDenom::Eden.as_str())
    }

    pub fn to_coin_available(&self) -> Coin {
        self.to_coin(self.available.clone().unwrap_or_default().amount)
    }

    pub fn to_coin_staked(&self) -> Coin {
        self.to_coin(self.staked.clone().unwrap_or_default().amount)
    }
}
// implement default
impl Default for EdenEarnProgram {
    fn default() -> Self {
        Self {
            bonding_period: 0,
            apr: AprElys::default(),
            available: None,
            staked: None,
            rewards: None,
            vesting: BalanceAvailable {
                amount: Uint128::zero(),
                usd_amount: Decimal::zero(),
            },
            vesting_details: None,
        }
    }
}
