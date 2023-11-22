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
    #[returns(InRouteByDenomResponse)]
    InRouteByDenom { denom_in: String, denom_out: String },
    #[returns(AmmSwapEstimationByDenomResponse)]
    AmmSwapEstimationByDenom {
        amount: Coin,
        denom_in: String,
        denom_out: String,
    },
    // Define OracleQuery
    #[returns(OracleAllPriceResponse)]
    OraclePriceAll { pagination: PageRequest },
    #[returns(OracleAssetInfoResponse)]
    OracleAssetInfo { denom: String },
    // Define MarginQuery
    #[returns(MarginQueryPositionsResponse)]
    MarginQueryPositions { pagination: PageRequest },
    #[returns(MarginMtpResponse)]
    MarginMtp { address: String, id: u64 },
    // Define AuthQuery
    #[returns(AuthAccountsResponse)]
    AuthAccounts { pagination: PageRequest },
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
    pub fn mtp(address: impl Into<String>, id: u64) -> Self {
        Self::MarginMtp {
            address: address.into(),
            id,
        }
    }
    pub fn positions(pagination: PageRequest) -> Self {
        Self::MarginQueryPositions { pagination }
    }
    pub fn accounts(pagination: PageRequest) -> Self {
        Self::AuthAccounts { pagination }
    }
    pub fn in_route_by_denom(denom_in: String, denom_out: String) -> Self {
        Self::InRouteByDenom {
            denom_in,
            denom_out,
        }
    }
    pub fn amm_swap_estimation_by_denom(amount: Coin, denom_in: String, denom_out: String) -> Self {
        Self::AmmSwapEstimationByDenom {
            amount,
            denom_in,
            denom_out,
        }
    }
}
