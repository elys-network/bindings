use crate::{
    entry_point::{execute, query, reply, sudo},
    msg::*,
    types::*,
    ContractError,
};

use cosmwasm_std::{coin, coins, Addr, Decimal, Event, Uint128};
use cw_multi_test::ContractWrapper;
mod get_order_id_from_events;
mod read_processed_order_id;
use cw_multi_test::Executor;
use elys_bindings_test::*;
use std::str::FromStr;

mod create_spot_order {
    use super::*;
    mod coin_number;
    mod not_enough_fund;
    mod order_price_denom;
    mod order_same_denom;
    mod order_wrong_fund;
    mod successful_create_limit_buy_order;
    mod successful_create_limit_sell_order;
    mod successful_create_market_order;
    mod successful_create_stop_loss_order;
}

mod cancel_spot_order {
    use super::*;
    mod not_found;
    mod process_spot_order_processing;
    mod successful_cancel_order_with_created_order;
    mod successful_cancel_order_with_dummy_order;
    mod unauthorized;
}

mod cancel_spot_orders {
    use super::*;
    mod successfully_cancel_orders;
    mod successfully_cancel_orders_id;
    mod successfully_cancel_orders_type;
    mod unauthorize;
}

mod get_spot_order {
    use super::*;
    use cosmwasm_std::{Binary, StdError};
    mod not_found;
    mod successful_query_message;
}

mod get_spot_orders {
    use super::*;
    mod get_spot_orders;
}

mod process_spot_order {
    use super::*;
    mod process_limit_buy_order_with_testnet_data;
    mod succesful_process_limit_buy_order;
    mod successful_process_5_of_10_orders;
    mod successful_process_limit_sell_order;
    mod successful_process_stop_loss_order;
}

mod create_perpetual_order {
    use super::*;
    mod coin_number;
    mod reproduce_testnet_issue_create_perpetual_market_open_order;
    mod successful_create_perpetual_market_close;
    mod successful_create_perpetual_market_order;
    mod successful_create_perpetual_order;
}

mod cancel_perpetual_order {
    use super::*;
    mod not_found;
    mod succesful_cancel_an_order;
    mod unauthorize;
}

mod process_perpetual_order {
    use super::*;
    mod process_limit_open;
    mod process_order_close;
}

mod get_perpetual_order {
    use super::*;

    mod not_found;
    mod successful_query_message;
}

mod close_perpetual_position {
    use super::*;
    mod closing_a_perpetual_position;
}

pub use mock::instantiate::*;
mod mock {
    pub mod instantiate;
}
