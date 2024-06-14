use crate::trade_shield::types::default_take_profit_price;
#[allow(unused_imports)]
use crate::types::{BalanceAvailable, PageRequest, SwapAmountInRoute};

#[allow(unused_imports)]
use super::query_resp::*;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, CustomQuery, Decimal, Int128, SignedDecimal, SignedDecimal256, Uint128};

// Now define ElysQuery to include the new OracleQuery and AmmQuery
#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    // Define AmmQuery
    #[returns(AmmSwapEstimationResponse)]
    AmmSwapEstimation {
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
        discount: Decimal,
    },
    #[returns(AmmSwapEstimationByDenomResponse)]
    AmmSwapEstimationByDenom {
        amount: Coin,
        denom_in: String,
        denom_out: String,
        discount: Decimal,
    },
    #[returns(BalanceAvailable)]
    AmmBalance { address: String, denom: String },
    #[returns(AmmGetPoolResponse)]
    AmmPool { pool_id: u64 },
    #[returns(AmmGetPoolsResponse)]
    AmmPoolAll { pagination: Option<PageRequest> },
    #[returns(Decimal)]
    AmmPriceByDenom { token_in: Coin, discount: Decimal },
    #[returns(QueryEarnPoolResponse)]
    AmmEarnMiningPoolAll {
        pool_ids: Option<Vec<u64>>,
        filter_type: i32,
        pagination: Option<PageRequest>,
    },
    #[returns(QueryJoinPoolEstimationResponse)]
    AmmJoinPoolEstimation { pool_id: u64, amounts_in: Vec<Coin> },
    #[returns(QueryExitPoolEstimationResponse)]
    AmmExitPoolEstimation {
        pool_id: u64,
        share_amount_in: Uint128,
        token_out_denom: String,
    },

    // Define AssetProfil
    #[returns(QueryGetEntryResponse)]
    AssetProfileEntry { base_denom: String },
    #[returns(QueryGetEntryAllResponse)]
    AssetProfileEntryAll { pagination: Option<PageRequest> },

    // Define AuthQuery
    #[returns(AuthAddressesResponse)]
    AuthAddresses { pagination: Option<PageRequest> },

    // Define Commitment
    #[returns(BalanceAvailable)]
    CommitmentStakedBalanceOfDenom { address: String, denom: String },
    #[returns(QueryStakedPositionResponse)]
    CommitmentStakedPositions { delegator_address: String },
    #[returns(QueryUnstakedPositionResponse)]
    CommitmentUnStakedPositions { delegator_address: String },
    #[returns(QueryDelegatorDelegationsResponse)]
    CommitmentDelegations { delegator_address: String },
    #[returns(QueryDelegatorUnbondingDelegationsResponse)]
    CommitmentUnbondingDelegations { delegator_address: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    CommitmentAllValidators { delegator_address: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    CommitmentDelegatorValidators { delegator_address: String },
    #[returns(QueryShowCommitmentsResponse)]
    CommitmentShowCommitments { creator: String },
    #[returns(QueryVestingInfoResponse)]
    CommitmentVestingInfo { address: String },
    #[returns(CommitmentNumberOfCommitmentsResponse)]
    CommitmentNumberOfCommitments {},

    // Define Incentive
    #[returns(QueryAprResponse)]
    IncentiveApr { withdraw_type: i32, denom: String },
    #[returns(QueryAprsResponse)]
    IncentiveAprs {},

    // Define Masterchef
    #[returns(MasterchefParamsResponse)]
    MasterchefParams {},
    #[returns(MasterchefPoolInfoResponse)]
    MasterchefPoolInfo { pool_id: u64 },
    #[returns(MasterchefUserPendingRewardResponse)]
    MasterchefUserPendingReward { user: String },
    #[returns(QueryPoolAprsResponse)]
    MasterchefPoolAprs { pool_ids: Vec<u64> },
    #[returns(QueryStableStakeAprResponse)]
    MasterchefStableStakeApr { denom: String },

    // Define Estaking
    #[returns(EstakingRewardsResponse)]
    EstakingRewards { address: String },

    // Define Leveragelp
    #[returns(LeveragelpParamsResponse)]
    LeveragelpParams {},
    #[returns(LeveragelpPositionsResponse)]
    LeveragelpQueryPositions { pagination: Option<PageRequest> },
    #[returns(LeveragelpPositionsResponse)]
    LeveragelpQueryPositionsByPool {
        amm_pool_id: u64,
        pagination: Option<PageRequest>,
    },
    #[returns(LeveragelpStatusResponse)]
    LeveragelpGetStatus {},
    #[returns(LeveragelpPositionsResponse)]
    LeveragelpQueryPositionsForAddress {
        address: String,
        pagination: Option<PageRequest>,
    },
    #[returns(LeveragelpWhitelistResponse)]
    LeveragelpGetWhitelist { pagination: Option<PageRequest> },
    #[returns(LeveragelpIsWhitelistedResponse)]
    LeveragelpIsWhitelisted { address: String },
    #[returns(LeveragelpPoolResponse)]
    LeveragelpPool { index: u64 },
    #[returns(LeveragelpPoolsResponse)]
    LeveragelpPools { pagination: Option<PageRequest> },
    #[returns(LeveragelpPositionResponse)]
    LeveragelpPosition { address: String, id: u64 },
    #[returns(LeveragelpOpenEstimationResponse)]
    LeveragelpOpenEstimation {
        collateral_asset: String,
        collateral_amount: Int128,
        amm_pool_id: u64,
        leverage: Decimal,
    },
    #[returns(LeveragelpCloseEstimationResponse)]
    LeveragelpCloseEstimation {
        owner: String,
        id: u64,
        lp_amount: Int128,
    },
    // Define Perpetual
    #[returns(PerpetualQueryPositionsResponse)]
    PerpetualQueryPositions { pagination: PageRequest },
    #[returns(PerpetualMtpResponse)]
    PerpetualMtp { address: String, id: u64 },
    #[returns(PerpetualOpenEstimationResponse)]
    PerpetualOpenEstimation {
        position: i32,
        leverage: SignedDecimal,
        trading_asset: String,
        collateral: Coin,
        take_profit_price: SignedDecimal256,
        discount: Decimal,
    },
    #[returns(PerpetualGetPositionsForAddressResponse)]
    PerpetualGetPositionsForAddress {
        address: String,
        pagination: Option<PageRequest>,
    },

    // Define Oracle
    #[returns(OracleAllPriceResponse)]
    OraclePriceAll { pagination: PageRequest },
    #[returns(OracleAssetInfoResponse)]
    OracleAssetInfo { denom: String },
    #[returns(QueryGetPriceResponse)]
    OraclePrice {
        asset: String,
        source: String,
        timestamp: u64,
    },

    // Define Stablestake
    #[returns(BalanceBorrowed)]
    StableStakeBalanceOfBorrow {},
    #[returns(StableStakeParamsResp)]
    StableStakeParams {},
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn amm_swap_estimation(
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
        discount: Decimal,
    ) -> Self {
        Self::AmmSwapEstimation {
            routes,
            token_in,
            discount,
        }
    }
    pub fn amm_get_pool(pool_id: u64) -> Self {
        Self::AmmPool { pool_id }
    }
    pub fn amm_get_pools(pagination: Option<PageRequest>) -> Self {
        Self::AmmPoolAll { pagination }
    }
    pub fn oracle_get_all_prices(pagination: PageRequest) -> Self {
        Self::OraclePriceAll { pagination }
    }
    pub fn oracle_asset_info(denom: String) -> Self {
        Self::OracleAssetInfo { denom }
    }
    pub fn mtp(address: impl Into<String>, id: u64) -> Self {
        Self::PerpetualMtp {
            address: address.into(),
            id,
        }
    }
    pub fn positions(pagination: PageRequest) -> Self {
        Self::PerpetualQueryPositions { pagination }
    }
    pub fn accounts(pagination: Option<PageRequest>) -> Self {
        Self::AuthAddresses { pagination }
    }

    pub fn amm_swap_estimation_by_denom(
        amount: Coin,
        denom_in: String,
        denom_out: String,
        discount: Decimal,
    ) -> Self {
        Self::AmmSwapEstimationByDenom {
            amount,
            denom_in,
            denom_out,
            discount,
        }
    }

    pub fn get_balance(address: String, denom: String) -> Self {
        Self::AmmBalance { address, denom }
    }
    pub fn perpetual_open_estimation(
        position: i32,
        leverage: SignedDecimal,
        trading_asset: String,
        collateral: Coin,
        take_profit_price: Option<SignedDecimal256>,
        discount: Decimal,
    ) -> Self {
        let take_profit_price = match take_profit_price {
            Some(price) => price,
            None => default_take_profit_price(),
        };
        Self::PerpetualOpenEstimation {
            position,
            leverage,
            trading_asset,
            collateral,
            take_profit_price,
            discount,
        }
    }
    pub fn get_asset_profile(base_denom: String) -> Self {
        Self::AssetProfileEntry { base_denom }
    }
    pub fn get_all_asset_profile(pagination: Option<PageRequest>) -> Self {
        Self::AssetProfileEntryAll { pagination }
    }
    pub fn perpetual_get_position_for_address(
        address: String,
        pagination: Option<PageRequest>,
    ) -> Self {
        Self::PerpetualGetPositionsForAddress {
            address,
            pagination,
        }
    }
    pub fn get_oracle_price(asset: String, source: String, timestamp: u64) -> Self {
        ElysQuery::OraclePrice {
            asset,
            source,
            timestamp,
        }
    }
    pub fn get_staked_balance(address: String, denom: String) -> Self {
        ElysQuery::CommitmentStakedBalanceOfDenom { address, denom }
    }
    pub fn get_amm_price_by_denom(token_in: Coin, discount: Decimal) -> Self {
        ElysQuery::AmmPriceByDenom { token_in, discount }
    }
    pub fn get_staked_positions(delegator_addr: String) -> Self {
        ElysQuery::CommitmentStakedPositions {
            delegator_address: delegator_addr,
        }
    }
    pub fn get_unstaked_positions(delegator_addr: String) -> Self {
        ElysQuery::CommitmentUnStakedPositions {
            delegator_address: delegator_addr,
        }
    }
    pub fn get_borrowed_balance() -> Self {
        ElysQuery::StableStakeBalanceOfBorrow {}
    }
    pub fn get_stable_stake_params() -> Self {
        ElysQuery::StableStakeParams {}
    }
    pub fn get_delegations(delegator_addr: String) -> Self {
        ElysQuery::CommitmentDelegations {
            delegator_address: delegator_addr,
        }
    }
    pub fn get_unbonding_delegations(delegator_addr: String) -> Self {
        ElysQuery::CommitmentUnbondingDelegations {
            delegator_address: delegator_addr,
        }
    }
    pub fn get_all_validators() -> Self {
        ElysQuery::CommitmentAllValidators {
            delegator_address: "".to_string(),
        }
    }
    pub fn get_delegator_validators(delegator_addr: String) -> Self {
        ElysQuery::CommitmentDelegatorValidators {
            delegator_address: delegator_addr,
        }
    }
    pub fn get_commitments(address: String) -> Self {
        ElysQuery::CommitmentShowCommitments { creator: address }
    }
    pub fn get_vesting_info(address: String) -> Self {
        ElysQuery::CommitmentVestingInfo { address }
    }
    pub fn get_incentive_apr(program: i32, denom: String) -> Self {
        ElysQuery::IncentiveApr {
            withdraw_type: program,
            denom,
        }
    }
    pub fn get_incentive_aprs() -> Self {
        ElysQuery::IncentiveAprs {}
    }
    pub fn get_all_pools(
        pool_ids: Option<Vec<u64>>,
        filter_type: i32,
        pagination: Option<PageRequest>,
    ) -> Self {
        ElysQuery::AmmEarnMiningPoolAll {
            pool_ids,
            filter_type,
            pagination,
        }
    }
    pub fn leveragelp_params() -> Self {
        Self::LeveragelpParams {}
    }
    pub fn leveragelp_query_positions(pagination: Option<PageRequest>) -> Self {
        Self::LeveragelpQueryPositions { pagination }
    }
    pub fn leveragelp_query_positions_by_pool(
        amm_pool_id: u64,
        pagination: Option<PageRequest>,
    ) -> Self {
        Self::LeveragelpQueryPositionsByPool {
            amm_pool_id,
            pagination,
        }
    }
    pub fn leveragelp_get_status() -> Self {
        Self::LeveragelpGetStatus {}
    }
    pub fn leveragelp_query_positions_for_address(
        address: String,
        pagination: Option<PageRequest>,
    ) -> Self {
        Self::LeveragelpQueryPositionsForAddress {
            address,
            pagination,
        }
    }
    pub fn leveragelp_get_whitelist(pagination: Option<PageRequest>) -> Self {
        Self::LeveragelpGetWhitelist { pagination }
    }
    pub fn leveragelp_is_whitelisted(address: String) -> Self {
        Self::LeveragelpIsWhitelisted { address }
    }
    pub fn leveragelp_pool(index: u64) -> Self {
        Self::LeveragelpPool { index }
    }
    pub fn leveragelp_pools(pagination: Option<PageRequest>) -> Self {
        Self::LeveragelpPools { pagination }
    }
    pub fn leveragelp_position(address: String, id: u64) -> Self {
        Self::LeveragelpPosition { address, id }
    }
    pub fn leveragelp_open_est(
        collateral_asset: String,
        collateral_amount: Int128,
        amm_pool_id: u64,
        leverage: Decimal,
    ) -> Self {
        Self::LeveragelpOpenEstimation {
            collateral_asset,
            collateral_amount,
            amm_pool_id,
            leverage,
        }
    }
    pub fn leveragelp_close_est(owner: String, id: u64, lp_amount: Int128) -> Self {
        Self::LeveragelpCloseEstimation {
            owner,
            id,
            lp_amount,
        }
    }

    pub fn join_pool_estimation(pool_id: u64, amounts_in: Vec<Coin>) -> Self {
        ElysQuery::AmmJoinPoolEstimation {
            pool_id,
            amounts_in,
        }
    }
    pub fn exit_pool_estimation(
        pool_id: u64,
        share_amount_in: Uint128,
        token_out_denom: String,
    ) -> Self {
        ElysQuery::AmmExitPoolEstimation {
            pool_id,
            share_amount_in,
            token_out_denom,
        }
    }

    pub fn masterchef_params() -> Self {
        Self::MasterchefParams {}
    }
    pub fn masterchef_pool_info(pool_id: u64) -> Self {
        Self::MasterchefPoolInfo { pool_id }
    }
    pub fn masterchef_pending_rewards(address: String) -> Self {
        Self::MasterchefUserPendingReward { user: address }
    }
    pub fn get_masterchef_pool_apr(pool_ids: Vec<u64>) -> Self {
        Self::MasterchefPoolAprs { pool_ids }
    }
    pub fn query_estaking_rewards(address: String) -> Self {
        ElysQuery::EstakingRewards { address }
    }
    pub fn get_masterchef_stable_stake_apr(denom: String) -> Self {
        Self::MasterchefStableStakeApr { denom }
    }
    pub fn commitment_number_of_commitments() -> Self {
        Self::CommitmentNumberOfCommitments {}
    }
}
