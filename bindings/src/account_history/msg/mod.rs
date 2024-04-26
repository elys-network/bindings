mod instantiate;
mod migration;
mod query;
mod sudo;

pub use instantiate::InstantiateMsg;
pub use migration::MigrationMsg;
pub use query::QueryMsg;
pub use sudo::SudoMsg;

pub mod query_resp {
    mod get_all_resp;
    mod get_portfolio_resp;
    mod get_rewards_resp;
    mod get_total_balance_resp;
    mod liquid_asset;
    mod membership_tier_response;
    mod params_resp;
    mod total_value_per_asset_resp;
    mod user_value_response;

    pub use get_all_resp::GetAllResp;
    pub use get_portfolio_resp::GetPortfolioResp;
    pub use get_rewards_resp::GetRewardsResp;
    pub use get_total_balance_resp::GetTotalBalanceResp;
    pub use liquid_asset::LiquidAsset;
    pub use membership_tier_response::MembershipTierResponse;
    pub use params_resp::ParamsResp;
    pub use total_value_per_asset_resp::GetLiquidAssetsResp;
    pub use user_value_response::UserValueResponse;

    mod staked_assets_response;
    pub use staked_assets_response::StakedAssetsResponse;
    pub use staked_assets_response::StakeAssetBalanceBreakdown;
    
    pub mod earn {
        mod get_eden_boost_earn_details_resp;
        pub use get_eden_boost_earn_details_resp::GetEdenBoostEarnProgramResp;
        mod get_eden_earn_details_resp;
        pub use get_eden_earn_details_resp::GetEdenEarnProgramResp;
        mod get_elys_earn_details_resp;
        pub use get_elys_earn_details_resp::GetElysEarnProgramResp;
        mod get_usdc_earn_details_resp;
        pub use get_usdc_earn_details_resp::GetUsdcEarnProgramResp;
    }

    pub mod estaking {
        mod get_estaking_rewards_response;
        pub use get_estaking_rewards_response::GetEstakingRewardsResponse;
    }

    pub mod masterchef {
        mod get_masterchef_pending_rewards;
        mod get_masterchef_claim_rewards;
        pub use get_masterchef_pending_rewards::GetMasterchefUserPendingRewardResponse;
        pub use get_masterchef_claim_rewards::GetMasterchefClaimRewardsResponse;
    }
}
