use crate::types::{PageRequest, SwapAmountInRoute};

#[allow(unused_imports)]
use super::query_resp::*;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, CustomQuery};

// Define OracleQuery
#[cw_serde]
#[derive(QueryResponses)]
pub enum OracleQuery {}

// Define AmmQuery
#[cw_serde]
pub enum AmmQuery {}

// Now define ElysQuery to include the new OracleQuery and AmmQuery
#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    #[returns(QuerySwapEstimationResponse)]
    QuerySwapEstimation {
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
    },
    #[returns(AllPriceResponse)]
    PriceAll { pagination: PageRequest },
    #[returns(OracleAssetInfoResponse)]
    OracleAssetInfo { denom: String },
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn swap_estimation(routes: Vec<SwapAmountInRoute>, token_in: Coin) -> Self {
        ElysQuery::QuerySwapEstimation { routes, token_in }
    }
}

impl ElysQuery {
    pub fn get_all_prices(pagination: PageRequest) -> Self {
        ElysQuery::PriceAll { pagination }
    }
    pub fn asset_info(denom: String) -> Self {
        ElysQuery::OracleAssetInfo { denom }
    }
}
