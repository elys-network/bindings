use crate::types::BalanceDollar;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetBalanceResp {
    pub balance: BalanceDollar,
}
