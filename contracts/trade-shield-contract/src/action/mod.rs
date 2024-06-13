use crate::{states::*, types::*, ContractError};
use cosmwasm_std::Event;
use cosmwasm_std::{BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Order, Response};
use elys_bindings::*;

pub mod query {
    mod asset_info;
    mod get_all_price;
    mod get_perpetual_order;
    mod get_perpetual_orders;
    mod get_perpetual_position;
    mod get_perpetual_positions;
    mod get_spot_order;
    mod get_spot_order_states;
    mod get_spot_orders;
    mod get_stat;
    mod perpetual_get_position_for_address;
    mod perpetual_open_estimation;
    mod swap_estimation_by_denom;

    use super::*;

    use crate::msg::query_resp::*;
    use cosmwasm_std::Deps;
    use elys_bindings::query_resp::*;

    pub use asset_info::asset_info;
    pub use get_all_price::get_all_prices;
    pub use get_perpetual_order::get_perpetual_order;
    pub use get_perpetual_orders::get_perpetual_orders;
    pub use get_perpetual_position::get_perpetual_position;
    pub use get_perpetual_positions::get_perpetual_positions;
    pub use get_spot_order::get_spot_order;
    pub use get_spot_order_states::get_spot_order_states;
    pub use get_spot_orders::get_spot_orders;
    pub use get_stat::get_stat;
    pub use perpetual_get_position_for_address::perpetual_get_position_for_address;
    pub use perpetual_open_estimation::perpetual_open_estimation;
    pub use swap_estimation_by_denom::swap_estimation_by_denom;
}

pub mod execute {
    mod cancel_perpetual_order;
    mod cancel_perpetual_orders;
    mod cancel_spot_order;
    mod cancel_spot_orders;
    mod close_perpetual_position;
    mod create_perpetual_order;
    mod create_spot_order;

    mod stake_request;
    mod unstake_request;

    mod claim_rewards_request;
    mod eden_cancel_vest_request;
    mod eden_claim_vesting_request;
    mod eden_vest_request;
    mod elys_cancel_unstake_request;
    mod elys_redelegation_request;
    mod exit_amm_pool_request;
    mod join_amm_pool_request;

    mod close_leveragelp_position_request;
    mod masterchef_claim_rewards;
    mod open_leveragelp_position_request;
    mod update_stop_loss_leverage;

    mod estaking_withdraw_elys_staking_rewards;
    mod estaking_withdraw_reward;

    use super::*;

    pub use cancel_perpetual_order::cancel_perpetual_order;
    pub use cancel_perpetual_orders::cancel_perpetual_orders;
    pub use cancel_spot_order::cancel_spot_order;
    pub use cancel_spot_orders::cancel_spot_orders;
    pub use close_perpetual_position::close_perpetual_position;
    pub use create_perpetual_order::create_perpetual_order;
    pub use create_spot_order::create_spot_order;

    pub use claim_rewards_request::claim_rewards_request;
    pub use close_leveragelp_position_request::close_leveragelp_position_request;
    pub use eden_cancel_vest_request::eden_cancel_vest_request;
    pub use eden_claim_vesting_request::eden_claim_vesting_request;
    pub use eden_vest_request::eden_vest_request;
    pub use elys_cancel_unstake_request::elys_cancel_unstake_request;
    pub use elys_redelegation_request::elys_redelegation_request;
    pub use exit_amm_pool_request::exit_amm_pool_request;
    pub use join_amm_pool_request::join_amm_pool_request;
    pub use open_leveragelp_position_request::open_leveragelp_position_request;
    pub use stake_request::stake_request;
    pub use unstake_request::unstake_request;
    pub use update_stop_loss_leverage::update_stop_loss_leveragelp_request;

    pub use estaking_withdraw_elys_staking_rewards::estaking_withdraw_elys_staking_rewards;
    pub use estaking_withdraw_reward::estaking_withdraw_reward;
    pub use masterchef_claim_rewards::masterchef_claim_rewards;
}

pub mod reply {
    use super::*;
    use elys_bindings::msg_resp::*;

    mod close_perpetual_position;
    mod create_perpetual_order_market_close;
    mod create_perpetual_order_market_open;
    mod open_perpetual_position;
    mod spot_order;
    mod spot_order_market;

    pub use close_perpetual_position::reply_to_close_perpetual_order;
    pub use create_perpetual_order_market_close::reply_to_create_perpetual_market_close;
    pub use create_perpetual_order_market_open::reply_to_create_perpetual_market_open;
    pub use open_perpetual_position::reply_to_open_perpetual_position;
    pub use spot_order::reply_to_spot_order;
    pub use spot_order_market::reply_to_spot_order_market;
}

pub mod sudo {
    use super::*;

    mod process_orders;
    pub use process_orders::process_orders;
}
