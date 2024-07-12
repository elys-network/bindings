use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct StorageSizeResp {
    pub user_address_queue_data_size: u128,
    pub history_data_size: u128,
    pub old_history_2_data_size: u128,
}
