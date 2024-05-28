use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    AddUserAddressToQueue {
        user_address: String,
    },
    ChangeParams {
        update_account_enabled: Option<bool>,
        processed_account_per_block: Option<u64>,
    },
    UpdateAccount {},
    CleanHistory {},
}
