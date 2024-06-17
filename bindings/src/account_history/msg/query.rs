#[allow(unused_imports)]
use super::super::types::{PerpetualAssets, PortfolioBalanceSnapshot};
#[allow(unused_imports)]
use super::query_resp::earn::*;
#[allow(unused_imports)]
use super::query_resp::estaking::*;
#[allow(unused_imports)]
use super::query_resp::masterchef::*;

#[allow(unused_imports)]
use super::query_resp::*;
#[allow(unused_imports)]
use crate::query_resp::QueryStableStakeAprResponse;
#[allow(unused_imports)]
use crate::query_resp::{
    AuthAddressesResponse, BalanceBorrowed, MasterchefParamsResponse, MasterchefPoolInfoResponse,
    PoolFilterType, QueryAprsResponse, QueryEarnPoolResponse, QueryExitPoolEstimationResponse,
    QueryJoinPoolEstimationResponse, QueryPoolAssetEstimationResponse, QueryStakedPositionResponse,
    QueryUnstakedPositionResponse, QueryUserPoolResponse, QueryVestingInfoResponse,
    StableStakeParamsData, StakedAvailable,
};
#[allow(unused_imports)]
use crate::types::{BalanceAvailable, PageRequest};
use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused_imports)]
use cosmwasm_std::Uint128;
#[cfg(feature = "debug")]
use cosmwasm_std::{Coin, DecCoin, Decimal};
use cw2::ContractVersion;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AuthAddressesResponse)]
    Accounts { pagination: Option<PageRequest> },
    #[returns(GetLiquidAssetsResp)]
    GetLiquidAssets { user_address: String },
    #[returns(StakedAssetsResponse)]
    GetStakedAssets { user_address: Option<String> },
    #[returns(QueryUserPoolResponse)]
    GetPoolBalances { user_address: String },
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

    #[returns(MasterchefParamsResponse)]
    GetMasterchefParams {},

    #[returns(MasterchefPoolInfoResponse)]
    GetMasterchefPoolInfo { pool_id: u64 },

    #[returns(GetMasterchefUserPendingRewardResponse)]
    GetMasterchefPendingRewards { address: String },

    #[returns(QueryStableStakeAprResponse)]
    GetMasterchefStableStakeApr { denom: String },

    #[returns(MasterChefPoolAprResponse)]
    GetMasterChefPoolApr { pool_ids: Vec<u64> },
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

    #[cfg(feature = "debug")]
    #[returns(GetEdenEarnProgramResp)]
    GetEdenEarnProgramDetails { address: String },

    #[cfg(feature = "debug")]
    #[returns(GetEdenBoostEarnProgramResp)]
    GetEdenBoostEarnProgramDetails { address: String },

    #[cfg(feature = "debug")]
    #[returns(GetElysEarnProgramResp)]
    GetElysEarnProgramDetails { address: String },

    #[cfg(feature = "debug")]
    #[returns(GetUsdcEarnProgramResp)]
    GetUsdcEarnProgramDetails { address: String },

    #[cfg(feature = "debug")]
    #[returns(QueryAprsResponse)]
    IncentiveAprs {},

    #[cfg(feature = "debug")]
    #[returns(Uint128)]
    AddressQueueSize {},

    #[cfg(feature = "debug")]
    #[returns(ContractVersion)]
    Version {},
}
