use cosmwasm_schema::cw_serde;
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {
    pub limit: u64,
    pub expiration: Expiration,
    pub value_denom: String,
    pub trade_shield_address: String,
}
