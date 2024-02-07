use cosmwasm_schema::cw_serde;
use cw_utils::{Duration, Expiration};

#[cw_serde]
pub struct InstantiateMsg {
    pub limit: u64,
    pub interval: Duration,
    pub expiration: Expiration,
    pub trade_shield_address: String,
}
