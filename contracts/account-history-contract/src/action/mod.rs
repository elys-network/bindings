use crate::states::*;
use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use elys_bindings::*;

pub mod sudo {
    use super::*;
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
    mod all;
    mod get_membership_tier;
    mod get_perpetual_asset;
    mod get_portfolio;
    mod get_rewards;
    mod get_total_balance;
    mod last_snapshot;
    mod params;
    mod user_snapshots;
    mod user_value;

    pub use all::all;
    pub use get_liquid_assets::get_liquid_assets;
    pub use last_snapshot::last_snapshot;
    pub use params::params;
    pub use user_snapshots::user_snapshots;
    pub use user_value::user_value;
    mod get_eden_boost_earn_program_details;
    pub use get_eden_boost_earn_program_details::get_eden_boost_earn_program_details;
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
