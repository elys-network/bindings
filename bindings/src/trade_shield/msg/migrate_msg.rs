use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct MigrateMsg {
    pub account_history_address: Option<String>,
}
