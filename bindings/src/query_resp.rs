use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Int128};

use crate::types::{
    BaseAccount, Mtp, OracleAssetInfo, PageResponse, Price, SwapAmountInRoute, SwapAmountOutRoute,
};

#[cw_serde]
pub struct OracleAllPriceResponse {
    pub price: Option<Vec<Price>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct AmmSwapEstimationResponse {
    pub spot_price: Decimal,
    pub token_out: Coin,
    pub discount: Decimal,
    pub swap_fee: Decimal,
    pub available_liquidity: Coin,
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
    pub mtp: Option<Mtp>,
}

#[cw_serde]
pub struct AuthAccountsResponse {
    pub accounts: Vec<BaseAccount>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct InRouteByDenomResponse {
    pub in_routes: Vec<SwapAmountInRoute>,
}

#[cw_serde]
pub struct AmmSwapEstimationByDenomResponse {
    pub in_route: Option<Vec<SwapAmountInRoute>>,
    pub out_route: Option<Vec<SwapAmountOutRoute>>,
    pub spot_price: Decimal,
    pub amount: Coin,
    pub discount: Decimal,
    pub swap_fee: Decimal,
    pub available_liquidity: Coin,
}

#[cw_serde]
pub struct MarginOpenEstimationResponse {
    pub position: i32,
    pub leverage: Decimal,
    pub trading_asset: String,
    pub collateral: Coin,
    pub min_collateral: Coin,
    pub valid_collateral: bool,
    pub position_size: Coin,
    pub swap_fee: Decimal,
    pub discount: Decimal,
    pub open_price: Decimal,
    pub take_profit_price: Decimal,
    pub liquidation_price: Decimal,
    pub estimated_pnl: Int128,
    pub available_liquidity: Coin,
}
