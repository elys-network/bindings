use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct MigrateMsg {
    pub account_history_address: Option<String>,
    pub num_executed_orders: u64,
    pub last_order_executed_spot: u64,
    pub last_order_executed_perp: u64,
}
