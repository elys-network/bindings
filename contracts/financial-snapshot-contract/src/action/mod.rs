pub mod query {
    pub mod pod {
        mod get_pod_liquid_asset;
        mod get_pod_liquid_assets;
        mod get_pod_liquidity_position;
        mod get_pod_liquidity_positions;
        mod get_pod_portfolio;
        mod get_pod_rewards;
        mod get_pod_total_balance;

        use cosmwasm_std::Deps;
        use crate::{states::PORTFOLIO, states::TOTAL_BALANCE, states::REWARDS, states::LIQUID_ASSETS, states::LIQUIDITY_POSITIONS, types::*, ContractError};
        pub use get_pod_liquid_asset::get_pod_liquid_asset;
        pub use get_pod_liquid_assets::get_pod_liquid_assets;
        pub use get_pod_liquidity_position::get_pod_liquidity_position;
        pub use get_pod_liquidity_positions::get_pod_liquidity_positions;
        pub use get_pod_portfolio::get_pod_portfolio;
        pub use get_pod_rewards::get_pod_rewards;
        pub use get_pod_total_balance::get_pod_total_balance;
    }

    pub mod earn {
        mod get_eden_boost_earn_program_details;
        mod get_eden_earn_program_details;
        mod get_elys_earn_program_details;
        mod get_del_validators;
        mod get_all_validators;
        mod get_usdc_earn_program_details;
        mod get_delegations;
        mod get_unbonding_delegations;
        mod get_commitments;
        mod get_pools;
        mod get_usdc_price;

        use cosmwasm_std::Deps;
        use crate::ContractError;
        pub use get_eden_boost_earn_program_details::get_eden_boost_earn_program_details;
        pub use get_eden_earn_program_details::get_eden_earn_program_details;
        pub use get_elys_earn_program_details::get_elys_earn_program_details;
        pub use get_del_validators::get_delegator_validators;
        pub use get_all_validators::get_all_validators;
        pub use get_usdc_earn_program_details::get_usdc_earn_program_details;
        pub use get_delegations::get_delegations;
        pub use get_unbonding_delegations::get_unbonding_delegations;
        pub use get_commitments::get_commitments;
        pub use get_pools::get_pools;
        pub use get_usdc_price::get_usdc_price;
    }
}
