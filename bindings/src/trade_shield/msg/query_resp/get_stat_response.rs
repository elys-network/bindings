use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetStatResponse {
    pub block_height: u64,
    pub number_of_address_on_the_chain: u64,
    pub number_of_executed_order: u64,
    pub number_of_pending_order: u64,
    pub number_of_address_in_commitment: u64,
}
