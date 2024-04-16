use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal};

use crate::{account_history::types::StakedAssets, trade_shield::types::UnstakedPosition};

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

impl BalanceBreakdown {
    pub fn calculate_total_unstaking_balance(staked_asset_elys: Option<Vec<UnstakedPosition>>) -> Decimal{
        if let Some(unstaked_positions) =  staked_asset_elys{
            let total_usd_amount = unstaked_positions.iter().fold(
                Decimal::zero(),
                |acc, position| {
                    // Accumulate the usd_amount from each UnstakedPosition
                    acc + position.unstaked.usd_amount
                },
            );
            total_usd_amount
        }else {
            Decimal::zero()
        }
    }
}
