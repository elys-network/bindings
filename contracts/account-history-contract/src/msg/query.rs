#[allow(unused_imports)]
use super::query_resp::*;
#[allow(unused_imports)]
use crate::types::{AccountSnapshot, PerpetualAssets};
use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused_imports)]
use elys_bindings::query_resp::AuthAddressesResponse;
use elys_bindings::types::PageRequest;

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
}
