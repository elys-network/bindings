pub mod sudo {
    mod update_account;
    pub use update_account::update_account;
}

pub mod query {
    mod get_liquid_assets;
    use crate::error::ContractError;
    mod get_membership_tier;
    mod get_perpetual_asset;
    mod get_pool_balances;
    mod get_pools;
    mod get_pools_apr;
    mod join_pool_estimation;
    mod pool_asset_estimation;
    mod exit_pool_estimation;
    mod get_portfolio;
    mod get_rewards;
    mod get_total_balance;
    mod get_estaking_rewards;
    mod get_masterchef_pending_rewards;

    #[cfg(feature = "debug")]
    mod all;
    #[cfg(feature = "debug")]
    mod last_snapshot;
    #[cfg(feature = "debug")]
    mod params;
    #[cfg(feature = "debug")]
    mod user_snapshots;
    #[cfg(feature = "debug")]
    mod user_value;

    #[cfg(feature = "debug")]
    pub use all::all;
    #[cfg(feature = "debug")]
    pub use last_snapshot::last_snapshot;
    #[cfg(feature = "debug")]
    pub use params::params;
    #[cfg(feature = "debug")]
    pub use user_snapshots::user_snapshots;
    #[cfg(feature = "debug")]
    pub use user_value::user_value;

    pub use get_pool_balances::get_pool_balances;
    pub use get_pools::get_pools;
    pub use get_pools_apr::get_pools_apr;
    pub use join_pool_estimation::join_pool_estimation;
    pub use pool_asset_estimation::pool_asset_estimation;
    pub use exit_pool_estimation::exit_pool_estimation;
    mod get_eden_boost_earn_program_details;
    pub use get_eden_boost_earn_program_details::get_eden_boost_earn_program_details;
    pub use get_liquid_assets::get_liquid_assets;
    mod get_eden_earn_program_details;
    pub use get_eden_earn_program_details::get_eden_earn_program_details;
    mod get_elys_earn_program_details;
    pub use get_elys_earn_program_details::get_elys_earn_program_details;
    mod get_usdc_earn_program_details;
    pub use get_usdc_earn_program_details::get_usdc_earn_program_details;
    mod get_staked_assets;
    pub use get_membership_tier::get_membership_tier;
    pub use get_perpetual_asset::get_perpetuals_assets;
    pub use get_portfolio::get_portfolio;
    pub use get_rewards::get_rewards;
    pub use get_staked_assets::get_staked_assets;
    pub use get_total_balance::get_total_balance;
    pub use get_estaking_rewards::get_estaking_rewards;
    pub use get_masterchef_pending_rewards::get_masterchef_pending_rewards;
}
