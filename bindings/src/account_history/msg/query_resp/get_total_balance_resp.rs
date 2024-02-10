use crate::account_history::types::TotalBalance;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetTotalBalanceResp {
    pub balances: TotalBalance,
}
