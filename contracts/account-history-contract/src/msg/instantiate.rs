use cosmwasm_schema::cw_serde;
use cw_utils::Expiration;
use elys_bindings::types::SwapAmountInRoute;

#[cw_serde]
pub struct InstantiateMsg {
    pub limit: u64,
    pub expiration: Expiration,
    pub amm_routes: Vec<SwapAmountInRoute>,
}
