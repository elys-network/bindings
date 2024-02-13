use crate::{account_history::types::Metadata, types::PageRequest};
use cosmwasm_schema::cw_serde;
use cw_utils::Expiration;

#[cw_serde]
pub struct ParamsResp {
    pub expiration: Expiration,
    pub pagination: PageRequest,
    pub trade_shield_address: Option<String>,
    pub metadata: Metadata,
}
