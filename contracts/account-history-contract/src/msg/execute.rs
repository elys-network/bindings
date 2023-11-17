use cosmwasm_schema::cw_serde;
use cw_utils::Expiration;

#[cw_serde]
pub enum ExecuteMsg {
    SetLimit { limit: u64 },
    SetExpiration { expiration: Expiration },
    UpdateAccounts {},
}
