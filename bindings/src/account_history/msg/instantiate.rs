use cosmwasm_schema::cw_serde;
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {
    pub limit: Option<u64>,
    pub expiration: Option<Expiration>,
    pub trade_shield_address: Option<String>,
}
