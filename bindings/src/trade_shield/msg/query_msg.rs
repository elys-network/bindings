#[allow(unused_imports)]
use super::query_resp::*;
#[allow(unused_imports)]
use crate::query_resp::*;
use crate::trade_shield::types::{PerpetualOrderType, SpotOrderType, Status};
use crate::types::{PageRequest, PerpetualPosition};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal, Int128, SignedDecimal, SignedDecimal256};
use cw2::ContractVersion;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetSpotOrderResp)]
    GetSpotOrder { order_id: u64 },
    #[returns(GetAllPricesResponse)]
    GetAllPrices { limit: u64 },
    #[returns(OracleAssetInfoResponse)]
    AssetInfo { denom: String },
    #[returns(GetPerpetualOrderResp)]
    GetPerpetualOrder { id: u64 },
    #[returns(GetSpotOrdersResp)]
    GetSpotOrders {
        pagination: Option<PageRequest>,
        order_owner: Option<String>,
        order_type: Option<SpotOrderType>,
        order_status: Option<Status>,
    },
    #[returns(GetPerpetualOrdersResp)]
    GetPerpetualOrders {
        pagination: Option<PageRequest>,
        order_owner: Option<String>,
        order_type: Option<PerpetualOrderType>,
        order_status: Option<Status>,
    },
    #[returns(AmmGetPoolResponse)]
    AmmGetPool { pool_id: u64 },
    #[returns(AmmGetPoolsResponse)]
    AmmGetPools { pagination: Option<PageRequest> },
    #[returns(AmmSwapEstimationByDenomResponse)]
    SwapEstimationByDenom {
        amount: Coin,
        denom_in: String,
        denom_out: String,
        user_address: Option<String>,
    },
    #[returns(PerpetualMtpResponse)]
    GetPerpetualPosition { id: u64, address: String },
    #[returns(PerpetualQueryPositionsResponse)]
    GetPerpetualPositions { pagination: PageRequest },
    #[returns(PerpetualOpenEstimationResponse)]
    PerpetualOpenEstimation {
        position: PerpetualPosition,
        leverage: SignedDecimal,
        trading_asset: String,
        collateral: Coin,
        take_profit_price: Option<SignedDecimal256>,
        user_address: Option<String>,
    },
    #[returns(PerpetualGetPositionsForAddressResponse)]
    PerpetualGetPositionsForAddress {
        address: String,
        pagination: Option<PageRequest>,
    },
    #[returns(LeveragelpParamsResponse)]
    LeveragelpParams {},
    #[returns(LeveragelpPositionsResponse)]
    LeveragelpQueryPositions { pagination: Option<PageRequest> },
    #[returns(LeveragelpPositionsResponse)]
    LeveragelpQueryPositionsByPool {
        amm_pool_id: u64,
        pagination: Option<PageRequest>,
    },
    #[returns(LeveragelpStatusResponse)]
    LeveragelpGetStatus {},
    #[returns(LeveragelpPositionsResponse)]
    LeveragelpQueryPositionsForAddress {
        address: String,
        pagination: Option<PageRequest>,
    },
    #[returns(LeveragelpWhitelistResponse)]
    LeveragelpGetWhitelist { pagination: Option<PageRequest> },
    #[returns(LeveragelpIsWhitelistedResponse)]
    LeveragelpIsWhitelisted { address: String },
    #[returns(LeveragelpPoolResponse)]
    LeveragelpPool { index: u64 },
    #[returns(LeveragelpPoolsResponse)]
    LeveragelpPools { pagination: Option<PageRequest> },
    #[returns(LeveragelpOpenEstimationResponse)]
    LeveragelpOpenEstimation {
        collateral_asset: String,
        collateral_amount: Int128,
        amm_pool_id: u64,
        leverage: Decimal,
    },
    #[returns(LeveragelpCloseEstimationResponse)]
    LeveragelpCloseEstimation {
        owner: String,
        id: u64,
        lp_amount: Int128,
    },
    #[returns(LeveragelpPositionResponse)]
    LeveragelpPosition { address: String, id: u64 },
    #[returns(TradeShieldParamsResponse)]
    GetParams {},
    #[returns(NumberOfPendingOrderResp)]
    NumberOfPendingOrder {},
    #[returns(GetSortedOrderListResp)]
    GetSortedOrderList {
        order_type: SpotOrderType,
        base_denom: String,
        quote_denom: String,
    },
    #[returns(GetStatResponse)]
    GetStat {},
    #[returns(GetSpotOrderStatesResp)]
    GetSpotOrderStates { order_id: u64 },
    #[returns(ContractVersion)]
    Version {},
}
