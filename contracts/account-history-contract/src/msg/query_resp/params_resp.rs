use cosmwasm_schema::cw_serde;
use cw_utils::Expiration;
use elys_bindings::types::PageRequest;

#[cw_serde]
pub struct ParamsResp {
    pub expiration: Expiration,
    pub pagination: PageRequest,
    pub value_denom: String,
}
