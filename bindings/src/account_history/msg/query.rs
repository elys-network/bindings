#[allow(unused_imports)]
use super::super::types::{PerpetualAssets, PortfolioBalanceSnapshot};
use super::query_resp::estaking::*;
use super::query_resp::masterchef::*;

#[allow(unused_imports)]
use super::query_resp::*;
#[allow(unused_imports)]
use crate::query_resp::{
    AuthAddressesResponse, BalanceBorrowed, EstakingRewardsResponse, PoolFilterType,
    QueryEarnPoolResponse, QueryExitPoolEstimationResponse, QueryIncentivePoolAprsResponse,
    QueryJoinPoolEstimationResponse, QueryPoolAssetEstimationResponse, QueryStakedPositionResponse,
    QueryUnstakedPositionResponse, QueryUserPoolResponse, QueryVestingInfoResponse,
    StableStakeParamsData, StakedAvailable
};
#[allow(unused_imports)]
use crate::types::{BalanceAvailable, PageRequest};
use cosmwasm_schema::{cw_serde, QueryResponses};
#[cfg(feature = "debug")]
use cosmwasm_std::{Coin, DecCoin, Decimal};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AuthAddressesResponse)]
    Accounts { pagination: Option<PageRequest> },
    #[returns(GetLiquidAssetsResp)]
    GetLiquidAssets { user_address: String },
    #[returns(StakedAssetsResponse)]
    GetStakedAssets { user_address: String },
    #[returns(GetPortfolioResp)]
    GetPortfolio { user_address: String },
    #[returns(QueryUserPoolResponse)]
    GetPoolBalances { user_address: String },
    #[returns(GetTotalBalanceResp)]
    GetTotalBalance { user_address: String },
    #[returns(GetRewardsResp)]
    GetRewards { user_address: String },
    #[returns(MembershipTierResponse)]
    GetMembershipTier { user_address: String },
    #[returns(PerpetualAssets)]
    GetPerpetualAssets { user_address: String },
    #[returns(Decimal)]
    GetAssetPrice { asset: String },
    #[returns(Decimal)]
    GetAssetPriceFromDenomInToDenomOut { denom_in: String, denom_out: String },

    #[returns(QueryEarnPoolResponse)]
    GetLiquidityPools {
        pool_ids: Option<Vec<u64>>,
        filter_type: PoolFilterType,
        pagination: Option<PageRequest>,
    },

    #[returns(QueryIncentivePoolAprsResponse)]
    GetLiquidityPoolsApr { pool_ids: Option<Vec<u64>> },

    #[returns(QueryJoinPoolEstimationResponse)]
    JoinPoolEstimation { pool_id: u64, amounts_in: Vec<Coin> },

    #[returns(QueryExitPoolEstimationResponse)]
    ExitPoolEstimation {
        pool_id: u64,
        exit_fiat_amount: Decimal,
    },

    #[returns(QueryPoolAssetEstimationResponse)]
    PoolAssetEstimation { pool_id: u64, amount: DecCoin },

    #[returns(GetEstakingRewardsResponse)]
    GetEstakingRewards { address: String },

    #[returns(GetMasterchefUserPendingRewardResponse)]
    GetMasterchefPendingRewards { address: String },

    #[returns(GetMasterchefClaimRewardsResponse)]
    GetMasterchefClaimRewards {
        sender: String,
        pool_ids: Vec<u64>,
    },
    // debug only
    #[cfg(feature = "debug")]
    #[returns(ParamsResp)]
    Params {},

    #[cfg(feature = "debug")]
    #[returns(PortfolioBalanceSnapshot)]
    LastSnapshot { user_address: String },

    #[cfg(feature = "debug")]
    #[returns(UserValueResponse)]
    UserValue { user_address: String },

    #[cfg(feature = "debug")]
    #[returns(Vec<(String, Vec<PortfolioBalanceSnapshot>)>)]
    All { pagination: Option<PageRequest> },

    #[cfg(feature = "debug")]
    #[returns(Vec<PortfolioBalanceSnapshot>)]
    UserSnapshots { user_address: String },

    #[cfg(feature = "debug")]
    #[returns(QueryStakedPositionResponse)]
    CommitmentStakedPositions { delegator_address: String },

    #[cfg(feature = "debug")]
    #[returns(QueryUnstakedPositionResponse)]
    CommitmentUnStakedPositions { delegator_address: String },

    #[cfg(feature = "debug")]
    #[returns(BalanceAvailable)]
    CommitmentRewardsSubBucketBalanceOfDenom {
        address: String,
        denom: String,
        program: i32,
    },

    #[cfg(feature = "debug")]
    #[returns(StakedAvailable)]
    CommitmentStakedBalanceOfDenom { address: String, denom: String },

    #[cfg(feature = "debug")]
    #[returns(BalanceBorrowed)]
    StableStakeBalanceOfBorrow {},

    #[cfg(feature = "debug")]
    #[returns(StableStakeParamsData)]
    StableStakeParams {},

    #[cfg(feature = "debug")]
    #[returns(QueryVestingInfoResponse)]
    CommitmentVestingInfo { address: String },

    #[cfg(feature = "debug")]
    #[returns(BalanceAvailable)]
    Balance { address: String, denom: String },

    #[cfg(feature = "debug")]
    #[returns(Decimal)]
    AmmPriceByDenom { token_in: Coin, discount: Decimal },
}
