#[allow(unused_imports)]
use crate::types::{BalanceAvailable, PageRequest, SwapAmountInRoute};

#[allow(unused_imports)]
use super::query_resp::*;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, CustomQuery, Decimal};

// Now define ElysQuery to include the new OracleQuery and AmmQuery
#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    // Define AmmQuery
    #[returns(AmmSwapEstimationResponse)]
    AmmSwapEstimation {
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
        discount: Decimal,
    },
    #[returns(AmmSwapEstimationByDenomResponse)]
    AmmSwapEstimationByDenom {
        amount: Coin,
        denom_in: String,
        denom_out: String,
        discount: Decimal,
    },
    #[returns(BalanceAvailable)]
    AmmBalance { address: String, denom: String },
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
    #[returns(MarginOpenEstimationResponse)]
    MarginOpenEstimation {
        position: i32,
        leverage: Decimal,
        trading_asset: String,
        collateral: Coin,
        take_profit_price: Decimal,
        discount: Decimal,
    },
    // Define AuthQuery
    #[returns(AuthAccountsResponse)]
    AuthAccounts { pagination: PageRequest },
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn amm_swap_estimation(
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
        discount: Decimal,
    ) -> Self {
        Self::AmmSwapEstimation {
            routes,
            token_in,
            discount,
        }
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

    pub fn amm_swap_estimation_by_denom(
        amount: Coin,
        denom_in: String,
        denom_out: String,
        discount: Decimal,
    ) -> Self {
        Self::AmmSwapEstimationByDenom {
            amount,
            denom_in,
            denom_out,
            discount,
        }
    }

    pub fn get_balance(address: String, denom: String) -> Self {
        ElysQuery::AmmBalance { address, denom }
    }
}
