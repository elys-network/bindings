use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

use crate::types::{AssetInfo, PageResponse, Price, MTP};

#[cw_serde]
pub struct AllPriceResponse {
    pub price: Vec<Price>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct QuerySwapEstimationResponse {
    pub spot_price: Decimal,
    pub token_out: Coin,
}

#[cw_serde]
pub struct AssetInfoResponse {
    pub asset_info: AssetInfo,
}

#[cw_serde]
pub struct PositionsResponse {
    mtps: MTP,
    pagination: PageResponse,
}

#[cw_serde]
pub struct MTPResponse {
    mtp: MTP,
}
