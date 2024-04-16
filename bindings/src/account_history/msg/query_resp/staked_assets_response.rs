use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal};

use crate::account_history::types::StakedAssets;

#[cw_serde]
pub struct BalanceBreakdown {
    pub staked: Decimal,
    pub rewards: Decimal,
    pub unstaking: Decimal,
    pub vesting: Decimal,
  }

#[cw_serde]
pub struct StakedAssetsResponse {
    pub total_staked_balance: DecCoin,
    pub staked_assets: StakedAssets,
    pub balance: Decimal,
    pub balance_break_down : BalanceBreakdown,
}
