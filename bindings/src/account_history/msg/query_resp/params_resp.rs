use crate::account_history::types::Metadata;
use cosmwasm_schema::cw_serde;
use cw_utils::Expiration;

#[cw_serde]
pub struct ParamsResp {
    pub expiration: Expiration,
    pub processed_account_per_block: u64,
    pub trade_shield_address: Option<String>,
    pub update_account_enabled: bool,
    pub metadata: Metadata,
}
