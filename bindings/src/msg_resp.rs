use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Int64};

#[cw_serde]
pub struct AmmSwapExactAmountInResp {
    pub token_out_amount: Int64,
    pub meta_data: Option<Binary>,
}

#[cw_serde]
pub struct MarginOpenResponse {
    pub id: u64,
    pub meta_data: Option<Binary>,
}

#[cw_serde]
pub struct MarginCloseResponse {
    pub id: u64,
    pub meta_data: Option<Binary>,
}
