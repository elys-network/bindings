#[allow(unused_imports)]
use super::query_resp::*;
#[allow(unused_imports)]
use crate::types::{AccountSnapshot, PerpetualAssets};
use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused_imports)]
use elys_bindings::query_resp::{
    AuthAddressesResponse, BalanceBorrowed, QueryStakedPositionResponse,
    QueryUnstakedPositionResponse, QueryVestingInfoResponse, StakedAvailable,
};
use elys_bindings::types::{BalanceAvailable, PageRequest};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserValueResponse)]
    UserValue { user_address: String },
    #[returns(AuthAddressesResponse)]
    Accounts { pagination: Option<PageRequest> },
    #[returns(Vec<(String, Vec<AccountSnapshot>)>)]
    All {},
    #[returns(AccountSnapshot)]
    LastSnapshot { user_address: String },
    #[returns(GetLiquidAssetsResp)]
    GetLiquidAssets { user_address: String },
    #[returns(StakedAssetsResponse)]
    GetStakedAssets { user_address: String },
    #[returns(ParamsResp)]
    Params {},
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

    // debug only
    #[returns(QueryStakedPositionResponse)]
    CommitmentStakedPositions { delegator_address: String },
    #[returns(QueryUnstakedPositionResponse)]
    CommitmentUnStakedPositions { delegator_address: String },
    #[returns(BalanceAvailable)]
    CommitmentRewardsSubBucketBalanceOfDenom {
        address: String,
        denom: String,
        program: i32,
    },
    #[returns(StakedAvailable)]
    CommitmentStakedBalanceOfDenom { address: String, denom: String },
    #[returns(BalanceBorrowed)]
    StableStakeBalanceOfBorrow { address: String },
    #[returns(QueryVestingInfoResponse)]
    CommitmentVestingInfo { address: String },
    #[returns(BalanceAvailable)]
    Balance { address: String, denom: String },
}
