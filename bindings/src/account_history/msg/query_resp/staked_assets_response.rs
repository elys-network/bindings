use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal};

use crate::account_history::types::StakedAssets;

#[cw_serde]
#[derive(Default)]
pub struct StakeAssetBalanceBreakdown {
    pub staked: Decimal,
    pub unstaking: Decimal,
    pub vesting: Decimal,
}

#[cw_serde]
pub struct StakedAssetsResponse {
    pub total_staked_balance: DecCoin,
    pub staked_assets: StakedAssets,
    pub total_balance: Decimal,
    pub balance_break_down : StakeAssetBalanceBreakdown,
}

impl StakeAssetBalanceBreakdown {
    pub fn total(&self) -> Decimal {
        let total = self.staked.checked_add(self.unstaking)
            .and_then(|sum| sum.checked_add(self.vesting));
        
        match total {
            Ok(result) => result,
            Err(_) => Decimal::zero(),
        }
    }
}
