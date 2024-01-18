use crate::states::*;
use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use elys_bindings::*;

pub mod sudo {
    use super::*;
    mod update_account;
    pub use update_account::custom_err;
    pub use update_account::update_account;
}

pub mod query {
    mod get_total_value_per_asset;
    use crate::error::ContractError;
    mod get_pod_portfolio;
    mod params;
    mod user_value;

    pub use get_total_value_per_asset::get_total_value_per_asset;
    pub use params::params;
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
    pub use get_pod_portfolio::get_pod_portfolio;
    pub use get_staked_assets::get_staked_assets;
}
