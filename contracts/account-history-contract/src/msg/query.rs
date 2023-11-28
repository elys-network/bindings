#[allow(unused_imports)]
use super::query_resp::UserValueResponse;
use cosmwasm_schema::{cw_serde, QueryResponses};
use elys_bindings::query_resp::AuthAccountsResponse;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserValueResponse)]
    UserValue { user_address: String },
    #[returns(AuthAccountsResponse)]
    Accounts {},
}
