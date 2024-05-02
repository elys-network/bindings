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
pub struct PerpetualOpenResponse {
    pub id: u64,
}

#[cw_serde]
pub struct PerpetualCloseResponse {
    pub id: u64,
    pub amount: Int128,
}

#[cw_serde]
pub struct MsgResponse {
    pub result: String,
}

#[cw_serde]
pub struct MsgJoinPoolResponse {
    pub share_amount_out: Int128,
    pub token_in: Vec<Coin>,
}

#[cw_serde]
pub struct MsgExitPoolResponse {
    pub token_out: Vec<Coin>,
}

#[cw_serde]
pub struct GetMasterchefClaimRewardsResponse {
    pub code: u64,
    pub result: String,
}
