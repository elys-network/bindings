use crate::types::{PageRequest, SwapAmountInRoute};

#[allow(unused_imports)]
use super::query_resp::*;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, CustomQuery};

// Define OracleQuery
#[cw_serde]
#[derive(QueryResponses)]
pub enum OracleQuery {
    #[returns(AllPriceResponse)]
    PriceAll { pagination: PageRequest },
    #[returns(AssetInfoResponse)]
    AssetInfo { denom: String },
}

impl OracleQuery {
    pub fn get_all_prices(pagination: PageRequest) -> Self {
        OracleQuery::PriceAll { pagination }
    }
    pub fn asset_info(denom: String) -> Self {
        OracleQuery::AssetInfo { denom }
    }
}

// Define AmmQuery
#[cw_serde]
#[derive(QueryResponses)]
pub enum AmmQuery {
    #[returns(QuerySwapEstimationResponse)]
    QuerySwapEstimation {
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
    },
}

impl AmmQuery {
    pub fn swap_estimation(routes: Vec<SwapAmountInRoute>, token_in: Coin) -> Self {
        AmmQuery::QuerySwapEstimation { routes, token_in }
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum MarginQuery {
    #[returns(PositionsResponse)]
    Positions { pagination: PageRequest },
    #[returns(MTPResponse)]
    MTP { address: String, id: u64 },
}

impl MarginQuery {
    pub fn mtp(address: impl Into<String>, id: u64) -> Self {
        Self::MTP {
            address: address.into(),
            id,
        }
    }
    pub fn positions(pagination: PageRequest) -> Self {
        Self::Positions { pagination }
    }
}

// Now define ElysQuery to include the new OracleQuery, AmmQuery, and MarginQuery
#[cw_serde]
pub enum ElysQuery {
    Oracle(OracleQuery),
    Amm(AmmQuery),
    Margin(MarginQuery),
}

impl CustomQuery for ElysQuery {}
