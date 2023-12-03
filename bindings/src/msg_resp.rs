use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Int128, Int64};

use crate::types::{SwapAmountInRoute, SwapAmountOutRoute};

#[cw_serde]
pub struct AmmSwapExactAmountInResp {
    pub token_out_amount: Int64,
    pub discount: Decimal,
    pub swap_fee: Decimal,
    pub recipient: String,
}

#[cw_serde]
pub struct AmmSwapByDenomResponse {
    pub amount: Coin,
    pub in_route: Option<Vec<SwapAmountInRoute>>,
    pub out_route: Option<Vec<SwapAmountOutRoute>>,
    pub spot_price: Decimal,
    pub swap_fee: Decimal,
    pub recipient: String,
    pub discount: Decimal,
}

#[cw_serde]
pub struct MarginOpenResponse {
    pub id: u64,
}

#[cw_serde]
pub struct MarginCloseResponse {
    pub id: u64,
    pub amount: Int128,
}

#[cw_serde]
pub struct MsgResponse {
    pub result: String,
}
