use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

use crate::types::{OracleAssetInfo, PageResponse, Price, MTP};

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
    pub mtps: Option<Vec<MTP>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct MarginMTPResponse {
    pub mtp: MTP,
}
