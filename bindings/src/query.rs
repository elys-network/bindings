#[allow(unused_imports)]
use crate::types::{BalanceAvailable, PageRequest, SwapAmountInRoute};

#[allow(unused_imports)]
use super::query_resp::*;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, CustomQuery, Decimal, SignedDecimal, SignedDecimal256};

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
    // Define OracleQuery
    #[returns(OracleAllPriceResponse)]
    OraclePriceAll { pagination: PageRequest },
    #[returns(OracleAssetInfoResponse)]
    OracleAssetInfo { denom: String },
    // Define MarginQuery
    #[returns(MarginQueryPositionsResponse)]
    MarginQueryPositions { pagination: PageRequest },
    #[returns(MarginMtpResponse)]
    MarginMtp { address: String, id: u64 },
    #[returns(MarginOpenEstimationResponse)]
    MarginOpenEstimation {
        position: i32,
        leverage: SignedDecimal,
        trading_asset: String,
        collateral: Coin,
        take_profit_price: SignedDecimal256,
        discount: Decimal,
    },
    #[returns(MarginGetPositionsForAddressResponse)]
    MarginGetPositionsForAddress {
        address: String,
        pagination: PageRequest,
    },
    // Define AuthQuery
    #[returns(AuthAddressesResponse)]
    AuthAddresses { pagination: Option<PageRequest> },
    #[returns(QueryGetEntryResponse)]
    AssetProfileEntry { base_denom: String },
    #[returns(QueryGetEntryAllResponse)]
    AssetProfileEntryAll { pagination: Option<PageRequest> },
    #[returns(QueryAprResponse)]
    IncentiveApr { withdraw_type: i32, denom: String },
    #[returns(QueryGetPriceResponse)]
    OraclePrice {
        asset: String,
        source: String,
        timestamp: u64,
    },
    #[returns(BalanceAvailable)]
    CommitmentRewardsSubBucketBalanceOfDenom {
        address: String,
        denom: String,
        program: i32,
    },
    #[returns(BalanceAvailable)]
    CommitmentStakedBalanceOfDenom { address: String, denom: String },
    #[returns(Decimal)]
    AmmPriceByDenom { token_in: Coin, discount: Decimal },
    #[returns(QueryStakedPositionResponse)]
    CommitmentStakedPositions { delegator_address: String },
    #[returns(QueryUnstakedPositionResponse)]
    CommitmentUnStakedPositions { delegator_address: String },
    #[returns(BalanceBorrowed)]
    StableStakeBalanceOfBorrow { address: String },
    #[returns(QueryDelegatorDelegationsResponse)]
    CommitmentDelegations { delegator_address: String },
    #[returns(QueryDelegatorUnbondingDelegationsResponse)]
    CommitmentUnbondingDelegations { delegator_address: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    CommitmentAllValidators { delegator_address: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    CommitmentDelegatorValidators { delegator_address: String },
    #[returns(BalanceAvailable)]
    CommitmentRewardsBalanceOfDenom { address: String, denom: String },
    #[returns(QueryShowCommitmentsResponse)]
    CommitmentShowCommitments { creator: String },
    #[returns(QueryVestingInfoResponse)]
    CommitmentVestingInfo { address: String },
    #[returns(QueryEarnPoolResponse)]
    AmmEarnMiningPoolAll {
        pool_ids: Option<Vec<u64>>,
        filter_type: i32,
        pagination: Option<PageRequest>,
    },
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
    pub fn oracle_get_all_prices(pagination: PageRequest) -> Self {
        Self::OraclePriceAll { pagination }
    }
    pub fn oracle_asset_info(denom: String) -> Self {
        Self::OracleAssetInfo { denom }
    }
    pub fn mtp(address: impl Into<String>, id: u64) -> Self {
        Self::MarginMtp {
            address: address.into(),
            id,
        }
    }
    pub fn positions(pagination: PageRequest) -> Self {
        Self::MarginQueryPositions { pagination }
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
    pub fn margin_open_estimation(
        position: i32,
        leverage: SignedDecimal,
        trading_asset: String,
        collateral: Coin,
        take_profit_price: SignedDecimal256,
        discount: Decimal,
    ) -> Self {
        Self::MarginOpenEstimation {
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
    pub fn margin_get_position_for_address(address: String, pagination: PageRequest) -> Self {
        Self::MarginGetPositionsForAddress {
            address,
            pagination,
        }
    }
    pub fn get_sub_bucket_rewards_balance(address: String, denom: String, program: i32) -> Self {
        ElysQuery::CommitmentRewardsSubBucketBalanceOfDenom {
            address,
            denom,
            program,
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
    pub fn get_borrowed_balance(address: String) -> Self {
        ElysQuery::StableStakeBalanceOfBorrow { address }
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
    pub fn get_rewards_balance(address: String, denom: String) -> Self {
        ElysQuery::CommitmentRewardsBalanceOfDenom { address, denom }
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
}
