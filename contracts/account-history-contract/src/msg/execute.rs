use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    SetLimit { limit: u64 },
}