#[allow(unused_imports)]
use crate::types::AccountValue;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<AccountValue>)]
    UserHistory { user_address: String },
}
