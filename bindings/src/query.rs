use crate::types::{PageRequest, SwapAmountInRoute};

#[allow(unused_imports)]
use super::query_resp::*;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, CustomQuery};

// Now define ElysQuery to include the new OracleQuery and AmmQuery
#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    // Define AmmQuery
    #[returns(AmmSwapEstimationResponse)]
    AmmSwapEstimation {
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
    },
    // Define OracleQuery
    #[returns(OraclePriceAllResponse)]
    OraclePriceAll { pagination: PageRequest },
    #[returns(OracleAssetInfoResponse)]
    OracleAssetInfo { denom: String },
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn amm_swap_estimation(routes: Vec<SwapAmountInRoute>, token_in: Coin) -> Self {
        Self::AmmSwapEstimation { routes, token_in }
    }
    pub fn oracle_get_all_prices(pagination: PageRequest) -> Self {
        Self::OraclePriceAll { pagination }
    }
    pub fn oracle_asset_info(denom: String) -> Self {
        Self::OracleAssetInfo { denom }
    }
}
