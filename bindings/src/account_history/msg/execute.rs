use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    AddUserAddressToQueue {
        user_address: String,
    },
    ChangeParams {
        update_account_enabled: Option<bool>,
        processed_account_per_block: Option<u64>,
        delete_old_data_enabled: Option<bool>,
        delete_epoch: Option<u64>,
    },
    CleanStorage {
        limit: u64,
    },
    CleanStorageBulk {},
}
