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

// Now define ElysQuery to include the new OracleQuery and AmmQuery
#[cw_serde]
pub enum ElysQuery {
    Oracle(OracleQuery),
    Amm(AmmQuery),
}

impl CustomQuery for ElysQuery {}
