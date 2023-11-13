use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

use crate::types::{OracleAssetInfo, PageResponse, Price};

#[cw_serde]
pub struct AllPriceResponse {
    pub price: Vec<Price>,
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
