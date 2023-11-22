use crate::types::AccountValue;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct UserValueResponse {
    pub value: AccountValue,
}
