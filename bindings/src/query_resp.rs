use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

use crate::types::{Mtp, OracleAssetInfo, PageResponse, Price};

#[cw_serde]
pub struct OracleAllPriceResponse {
    pub price: Option<Vec<Price>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct AmmSwapEstimationResponse {
    pub spot_price: Decimal,
    pub token_out: Coin,
}

#[cw_serde]
pub struct OracleAssetInfoResponse {
    pub asset_info: OracleAssetInfo,
}

#[cw_serde]
pub struct MarginQueryPositionsResponse {
    pub mtps: Option<Vec<Mtp>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct MarginMtpResponse {
    pub mtp: Mtp,
}
