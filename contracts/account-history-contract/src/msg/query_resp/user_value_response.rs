use crate::types::AccountSnapshot;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct UserValueResponse {
    pub value: AccountSnapshot,
}
