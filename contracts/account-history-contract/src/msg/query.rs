#[allow(unused_imports)]
use super::query_resp::*;
#[allow(unused_imports)]
use crate::types::AccountSnapshot;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::DecCoin;
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
    #[returns(DecCoin)]
    TotalLiquidAssetBalance { user_address: String },
    #[returns(DecCoin)]
    TotalInOrdersBalance { user_address: String },
    #[returns(DecCoin)]
    TotalAvailableBalance { user_address: String },
    #[returns(TotalValueOfAssetResp)]
    TotalValueOfAsset { user_address: String, asset: String },
    #[returns(TotalValuePerAssetResp)]
    TotalValuePerAsset { user_address: String },
}
