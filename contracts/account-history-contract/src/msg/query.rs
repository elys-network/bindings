#[allow(unused_imports)]
use super::query_resp::{UserValueResponse, UserRewardsResponse};
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
    #[returns(UserRewardsResponse)]
    UserRewards { user_address: String },
}
