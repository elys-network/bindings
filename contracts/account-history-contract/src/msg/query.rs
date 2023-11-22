#[allow(unused_imports)]
use super::query_resp::UserValueResponse;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserValueResponse)]
    UserValue { user_address: String },
}
