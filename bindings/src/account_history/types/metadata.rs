use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdError, StdResult};

use crate::{query_resp::QueryAprResponse, trade_shield::types::EarnType, ElysQuerier};

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
}

impl Metadata {
    pub fn collect(querier: &ElysQuerier) -> StdResult<Self> {
        let usdc_denom_entry = querier
            .get_asset_profile(ElysDenom::Usdc.as_str().to_string())
            .map_err(|_| StdError::generic_err("an error occurred while getting usdc denom"))?;
        let usdc_denom = usdc_denom_entry.entry.denom;
        let usdc_base_denom = usdc_denom_entry.entry.base_denom;
        let usdc_display_denom = usdc_denom_entry.entry.display_name;
        let usdc_decimal = u64::checked_pow(10, usdc_denom_entry.entry.decimals as u32).unwrap();

        let eden_denom_entry = querier
            .get_asset_profile(ElysDenom::Eden.as_str().to_string())
            .map_err(|_| StdError::generic_err("an error occurred while getting eden denom"))?;

        Ok(Self {
            usdc_denom,
            usdc_base_denom,
            usdc_display_denom,
            usdc_decimal,
            eden_decimal: u64::checked_pow(10, eden_denom_entry.entry.decimals as u32).unwrap(),

            // APR section
            usdc_apr_usdc: querier
                .get_incentive_apr(
                    EarnType::UsdcProgram as i32,
                    ElysDenom::Usdc.as_str().to_string(),
                )
                .map_err(|_| {
                    StdError::generic_err("an error occurred while getting usdc apr in usdc")
                })?,
            eden_apr_usdc: querier
                .get_incentive_apr(
                    EarnType::UsdcProgram as i32,
                    ElysDenom::Eden.as_str().to_string(),
                )
                .map_err(|_| {
                    StdError::generic_err("an error occurred while getting eden apr in usdc")
                })?,

            usdc_apr_edenb: querier
                .get_incentive_apr(
                    EarnType::EdenBProgram as i32,
                    ElysDenom::Usdc.as_str().to_string(),
                )
                .map_err(|_| {
                    StdError::generic_err("an error occurred while getting usdc apr in edenb")
                })?,
            eden_apr_edenb: querier
                .get_incentive_apr(
                    EarnType::EdenBProgram as i32,
                    ElysDenom::Eden.as_str().to_string(),
                )
                .map_err(|_| {
                    StdError::generic_err("an error occurred while getting eden apr in edenb")
                })?,

            usdc_apr_eden: querier
                .get_incentive_apr(
                    EarnType::EdenProgram as i32,
                    ElysDenom::Usdc.as_str().to_string(),
                )
                .map_err(|_| {
                    StdError::generic_err("an error occurred while getting usdc apr in eden")
                })?,
            eden_apr_eden: querier
                .get_incentive_apr(
                    EarnType::EdenProgram as i32,
                    ElysDenom::Eden.as_str().to_string(),
                )
                .map_err(|_| {
                    StdError::generic_err("an error occurred while getting eden apr in eden")
                })?,
            edenb_apr_eden: querier
                .get_incentive_apr(
                    EarnType::EdenProgram as i32,
                    ElysDenom::EdenBoost.as_str().to_string(),
                )
                .map_err(|_| {
                    StdError::generic_err("an error occurred while getting edenb apr in eden")
                })?,

            usdc_apr_elys: querier
                .get_incentive_apr(
                    EarnType::ElysProgram as i32,
                    ElysDenom::Usdc.as_str().to_string(),
                )
                .map_err(|_| {
                    StdError::generic_err("an error occurred while getting usdc apr in elys")
                })?,
            eden_apr_elys: querier
                .get_incentive_apr(
                    EarnType::ElysProgram as i32,
                    ElysDenom::Eden.as_str().to_string(),
                )
                .map_err(|_| {
                    StdError::generic_err("an error occurred while getting eden apr in elys")
                })?,
            edenb_apr_elys: querier
                .get_incentive_apr(
                    EarnType::ElysProgram as i32,
                    ElysDenom::EdenBoost.as_str().to_string(),
                )
                .map_err(|_| {
                    StdError::generic_err("an error occurred while getting edenb apr in elys")
                })?,
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
        }
    }
}
