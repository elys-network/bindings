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
    mod get_total_value_ot_asset;
    mod get_total_value_per_asset;
    mod user_value;

    pub use get_total_value_ot_asset::get_total_value_of_asset;
    pub use get_total_value_per_asset::get_total_value_per_asset;
    pub use user_value::user_value;
}
