use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    AddUserAddressToQueue { user_address: String },
}
