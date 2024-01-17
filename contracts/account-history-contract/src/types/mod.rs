mod account_snapshot;
mod coin_value;

pub use account_snapshot::{StakedAsset, AccountSnapshot, StakedAssetResponse};
pub use coin_value::CoinValue;

pub mod earn_detail {
    pub mod earn_detail;
}
pub use earn_detail::earn_detail::{AprUsdc, AprElys, BalanceBorrowed, BalanceReward, VestingDetail, StakingValidator, ValidatorDetail, QueryAprResponse};

pub mod earn_program {
    pub mod eden_boost_earn;
    pub use eden_boost_earn::EdenBoostEarnProgram;

    pub mod eden_earn;
    pub use eden_earn::EdenEarnProgram;

    pub mod elys_earn;
    pub use elys_earn::ElysEarnProgram;

    pub mod usdc_earn;
    pub use usdc_earn::UsdcEarnProgram;
}

pub mod denom;
pub use denom::ElysDenom;

use elys_bindings::types::BalanceAvailable;