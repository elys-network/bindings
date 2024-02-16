use crate::query_resp::UserPoolResp;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct PoolBalances {
    pub balances: Vec<UserPoolResp>,
}

impl Default for PoolBalances {
    fn default() -> Self {
        Self { balances: vec![] }
    }
}
