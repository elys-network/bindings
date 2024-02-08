pub mod sudo {
    mod update_account;
    pub use update_account::update_account;
    mod get_all_orders;
    mod get_perpetuals;
    mod get_rewards;
    mod get_staked_assets;
}

pub mod query {
    mod get_liquid_assets;
    use crate::error::ContractError;
    mod get_membership_tier;
    mod get_perpetual_asset;
    mod get_portfolio;
    mod get_rewards;
    mod get_total_balance;

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
}
