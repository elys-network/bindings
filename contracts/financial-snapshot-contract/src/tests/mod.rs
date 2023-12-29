use crate::{
    entry_point::{query},
    msg::*,
    ContractError,
};

mod get_portfolio {
    use super::*;
    use cosmwasm_std::{Binary, StdError};
}

pub use mock::instantiate::*;
mod mock {
    #[allow(dead_code, unused)]
    pub mod instantiate;
}
