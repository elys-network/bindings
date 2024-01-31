#[allow(unused_imports)]
use super::query_resp::*;
#[allow(unused_imports)]
use crate::query_resp::*;
use crate::trade_shield::types::{PerpetualOrderType, SpotOrderType, Status};
use crate::types::{PageRequest, PerpetualPosition};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, SignedDecimal, SignedDecimal256};

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
}
