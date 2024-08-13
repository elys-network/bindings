use crate::account_history::types::PortfolioBalanceSnapshot;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct UserValueResponse {
    pub value: PortfolioBalanceSnapshot,
}
