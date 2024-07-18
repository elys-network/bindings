mod execute;
mod instantiate;
mod migration;
mod query;
mod sudo;

pub use execute::ExecuteMsg;
pub use instantiate::InstantiateMsg;
pub use migration::MigrationMsg;
pub use query::QueryMsg;
pub use sudo::SudoMsg;

pub mod query_resp {
    mod get_all_resp;
    mod get_rewards_resp;
    mod get_storage_size;
    mod liquid_asset;
    mod params_resp;
    mod total_value_per_asset_resp;
    mod user_value_response;

    pub use get_all_resp::GetAllResp;
    pub use get_rewards_resp::GetRewardsResp;
    pub use get_storage_size::StorageSizeResp;
    pub use liquid_asset::LiquidAsset;
    pub use params_resp::ParamsResp;
    pub use total_value_per_asset_resp::GetLiquidAssetsResp;
    pub use user_value_response::UserValueResponse;

    mod staked_assets_response;
    pub use staked_assets_response::StakeAssetBalanceBreakdown;
    pub use staked_assets_response::StakedAssetsResponse;

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
        mod get_masterchef_pool_apr_response;
        mod get_masterchef_stable_stake_response;
        pub use get_masterchef_pending_rewards::GetMasterchefUserPendingRewardResponse;
        pub use get_masterchef_pool_apr_response::MasterChefPoolAprResponse;
        pub use get_masterchef_stable_stake_response::StableStakeAprResponse;
    }
}
