#[allow(unused_imports)]
use super::super::types::{AccountSnapshot, PerpetualAssets};
#[allow(unused_imports)]
use super::query_resp::*;
#[allow(unused_imports)]
use crate::query_resp::{
    AuthAddressesResponse, BalanceBorrowed, PoolFilterType, QueryEarnPoolResponse,
    QueryStakedPositionResponse, QueryUnstakedPositionResponse, QueryVestingInfoResponse,
    StableStakeParamsData, StakedAvailable,
};
#[allow(unused_imports)]
use crate::types::{BalanceAvailable, PageRequest};
use cosmwasm_schema::{cw_serde, QueryResponses};
#[cfg(feature = "debug")]
use cosmwasm_std::{Coin, Decimal};

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

    // debug only
    #[cfg(feature = "debug")]
    #[returns(ParamsResp)]
    Params {},

    #[cfg(feature = "debug")]
    #[returns(AccountSnapshot)]
    LastSnapshot { user_address: String },

    #[cfg(feature = "debug")]
    #[returns(UserValueResponse)]
    UserValue { user_address: String },

    #[cfg(feature = "debug")]
    #[returns(Vec<(String, Vec<AccountSnapshot>)>)]
    All { pagination: Option<PageRequest> },

    #[cfg(feature = "debug")]
    #[returns(Vec<AccountSnapshot>)]
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
