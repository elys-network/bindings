use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct MigrationMsg {
    pub trade_shield_address: String,
}
