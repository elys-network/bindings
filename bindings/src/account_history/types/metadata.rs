use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, StdError, StdResult, Uint128};

use crate::{query_resp::QueryAprResponse, ElysQuerier};

use super::ElysDenom;

#[cw_serde]
pub struct Metadata {
    pub usdc_denom: String,
    pub usdc_base_denom: String,
    pub usdc_display_denom: String,
    pub usdc_decimal: u64,
    pub eden_decimal: u64,
    pub usdc_apr_usdc: QueryAprResponse,
    pub eden_apr_usdc: QueryAprResponse,
    pub usdc_apr_edenb: QueryAprResponse,
    pub eden_apr_edenb: QueryAprResponse,
    pub usdc_apr_eden: QueryAprResponse,
    pub eden_apr_eden: QueryAprResponse,
    pub edenb_apr_eden: QueryAprResponse,
    pub usdc_apr_elys: QueryAprResponse,
    pub eden_apr_elys: QueryAprResponse,
    pub edenb_apr_elys: QueryAprResponse,
    pub uusdc_usd_price: Decimal,
    pub uelys_price_in_uusdc: Decimal,
}

impl Metadata {
    pub fn update_prices(&self, querier: &ElysQuerier) -> StdResult<Self> {
        let usdc_oracle_price = querier
            .get_oracle_price(
                self.usdc_display_denom.clone(),
                ElysDenom::AnySource.as_str().to_string(),
                0,
            )
            .map_err(|_| StdError::generic_err("an error occurred while getting usdc price"))?;
        let uusdc_usd_price = usdc_oracle_price
            .price
            .price
            .checked_div(
                Decimal::from_atomics(Uint128::new(self.usdc_decimal as u128), 0)
                    .map_or(Decimal::zero(), |res| res),
            )
            .map_or(Decimal::zero(), |res| res);
        let uelys_price_in_uusdc = querier.get_asset_price(ElysDenom::Elys.as_str())?;

        Ok(Self {
            uusdc_usd_price,
            uelys_price_in_uusdc,
            ..self.clone()
        })
    }

    pub fn collect(querier: &ElysQuerier) -> StdResult<Self> {
        let usdc_denom_entry = querier
            .get_asset_profile(ElysDenom::Usdc.as_str().to_string())
            .map_err(|_| StdError::generic_err("an error occurred while getting usdc denom"))?;
        let usdc_denom = usdc_denom_entry.entry.denom;
        let usdc_base_denom = usdc_denom_entry.entry.base_denom;
        let usdc_display_denom = usdc_denom_entry.entry.display_name;
        let usdc_decimal =
            u64::checked_pow(10, usdc_denom_entry.entry.decimals as u32).map_or(0, |res| res);

        let eden_denom_entry = querier
            .get_asset_profile(ElysDenom::Eden.as_str().to_string())
            .map_err(|_| StdError::generic_err("an error occurred while getting eden denom"))?;

        // panic!("LALAL");
        let aprs = querier.get_incentive_aprs()?;

        Ok(Self {
            usdc_denom,
            usdc_base_denom,
            usdc_display_denom,
            usdc_decimal,
            eden_decimal: u64::checked_pow(10, eden_denom_entry.entry.decimals as u32)
                .map_or(0, |res| res),

            // APR section
            usdc_apr_usdc: QueryAprResponse {
                apr: aprs.usdc_apr_usdc,
            },
            eden_apr_usdc: QueryAprResponse {
                apr: aprs.eden_apr_usdc,
            },

            usdc_apr_edenb: QueryAprResponse {
                apr: aprs.usdc_apr_edenb,
            },
            eden_apr_edenb: QueryAprResponse {
                apr: aprs.eden_apr_edenb,
            },

            usdc_apr_eden: QueryAprResponse {
                apr: aprs.usdc_apr_eden,
            },
            eden_apr_eden: QueryAprResponse {
                apr: aprs.eden_apr_eden,
            },
            edenb_apr_eden: QueryAprResponse {
                apr: aprs.edenb_apr_eden,
            },

            usdc_apr_elys: QueryAprResponse {
                apr: aprs.usdc_apr_elys,
            },
            eden_apr_elys: QueryAprResponse {
                apr: aprs.eden_apr_elys,
            },
            edenb_apr_elys: QueryAprResponse {
                apr: aprs.edenb_apr_elys,
            },

            // prices
            uusdc_usd_price: Decimal::zero(),
            uelys_price_in_uusdc: Decimal::zero(),
        })
    }
}

// default
impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            usdc_denom: "usdc".to_string(),
            usdc_base_denom: "uusdc".to_string(),
            usdc_display_denom: "USDC".to_string(),
            usdc_decimal: 6,
            eden_decimal: 6,
            usdc_apr_usdc: QueryAprResponse::default(),
            eden_apr_usdc: QueryAprResponse::default(),
            usdc_apr_edenb: QueryAprResponse::default(),
            eden_apr_edenb: QueryAprResponse::default(),
            usdc_apr_eden: QueryAprResponse::default(),
            eden_apr_eden: QueryAprResponse::default(),
            edenb_apr_eden: QueryAprResponse::default(),
            usdc_apr_elys: QueryAprResponse::default(),
            eden_apr_elys: QueryAprResponse::default(),
            edenb_apr_elys: QueryAprResponse::default(),
            uusdc_usd_price: Decimal::zero(),
            uelys_price_in_uusdc: Decimal::zero(),
        }
    }
}
