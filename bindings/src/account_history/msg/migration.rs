use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct MigrationMsg {
    pub limit: Option<u64>,
    pub trade_shield_address: Option<String>,
}
