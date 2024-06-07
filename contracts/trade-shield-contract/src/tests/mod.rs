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
mod test_order_status;
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
    mod pending_limit_buy_order_with_price_not_met;
    mod process_limit_buy_order_with_executed_status;
    mod process_limit_buy_order_with_executed_status_scenario_2;
    mod process_limit_buy_order_with_pending_status;
    mod process_order_limit_amount;
    mod successful_process_5_of_10_orders;
    mod successful_process_limit_sell_order;
    mod successful_process_stop_loss_order;
}

mod create_perpetual_order {
    use super::*;
    mod change_trigger_price;
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
    mod pending_limit_open_long_with_price_met;
    mod pending_limit_open_long_with_price_not_met;
    mod pending_limit_open_short_with_price_met;
    mod pending_limit_open_short_with_price_not_met;
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

mod stake_error_handling {
    use super::*;
    mod eden_cancel_vest_request;
    mod eden_vest_request;
    mod elys_cancel_unstake_request;
    mod elys_redelegation_request;
    mod stake_request;
    mod unstake_request;
}

mod claim_rewards_request {
    use super::*;
    mod claim_rewards_request;
}

mod leveragelp_open {
    use super::*;
    mod invalid_collateral;
    mod invalid_leverage;
}

pub use mock::instantiate::*;
mod mock {
    pub mod instantiate;
}
