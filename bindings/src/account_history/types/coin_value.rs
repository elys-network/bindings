use crate::{query_resp::OracleAssetInfoResponse, types::OracleAssetInfo, ElysQuerier};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, StdError, StdResult};

use super::ElysDenom;

#[cw_serde]
#[derive(Default)]
pub struct CoinValue {
    pub denom: String,
    pub amount_token: Decimal,
    pub price: Decimal,
    pub amount_usd: Decimal,
}

impl CoinValue {
    pub fn new(denom: String, amount_token: Decimal, price: Decimal, amount_usd: Decimal) -> Self {
        Self {
            denom,
            amount_token,
            price,
            amount_usd,
        }
    }

    pub fn from_price_and_coin(
        balance: &Coin,
        price_and_decimal_point: (Decimal, u64),
    ) -> StdResult<Self> {
        let amount_token = Decimal::from_atomics(balance.amount, price_and_decimal_point.1 as u32)
            .map_err(|e| {
                StdError::generic_err(format!("failed to convert amount to Decimal: {}", e))
            })?;

        let amount_usd = price_and_decimal_point
            .0
            .clone()
            .checked_mul(
                Decimal::from_atomics(balance.amount, price_and_decimal_point.1 as u32).map_err(
                    |e| {
                        StdError::generic_err(format!(
                            "failed to convert amount_usd_base to Decimal: {}",
                            e
                        ))
                    },
                )?,
            )
            .map_err(|e| {
                StdError::generic_err(format!(
                    "failed to convert amount_usd_base to Decimal: {}",
                    e
                ))
            })?;

        Ok(Self {
            denom: balance.denom.clone(),
            amount_token,
            price: price_and_decimal_point.0,
            amount_usd,
        })
    }

    pub fn from_coin(balance: &Coin, querier: &ElysQuerier<'_>) -> StdResult<Self> {
        let OracleAssetInfoResponse { asset_info } = querier
            .asset_info(balance.denom.clone())
            .unwrap_or(OracleAssetInfoResponse {
                asset_info: OracleAssetInfo {
                    denom: balance.denom.clone(),
                    display: balance.denom.clone(),
                    band_ticker: balance.denom.clone(),
                    elys_ticker: balance.denom.clone(),
                    decimal: 6,
                },
            });
        let decimal_point_usd = asset_info.decimal;

        let amount_token = Decimal::from_atomics(balance.amount, decimal_point_usd as u32)
            .map_err(|e| {
                StdError::generic_err(format!("failed to convert amount to Decimal: {}", e))
            })?;

        // Eden boost does not have Fiat valuation.
        if balance.denom == ElysDenom::EdenBoost.as_str() {
            return Ok(Self {
                amount_token,
                denom: balance.denom.clone(),
                amount_usd: Decimal::zero(),
                price: Decimal::zero(),
            });
        }

        let price = querier
            .get_asset_price(balance.denom.clone())
            .map_err(|e| StdError::generic_err(format!("failed to get_asset_price: {}", e)))?;

        let amount_usd = price
            .clone()
            .checked_mul(
                Decimal::from_atomics(balance.amount, decimal_point_usd as u32).map_err(|e| {
                    StdError::generic_err(format!(
                        "failed to convert amount_usd_base to Decimal: {}",
                        e
                    ))
                })?,
            )
            .map_err(|e| {
                StdError::generic_err(format!(
                    "failed to convert amount_usd_base to Decimal: {}",
                    e
                ))
            })?;

        Ok(Self {
            denom: balance.denom.clone(),
            amount_token,
            price,
            amount_usd,
        })
    }
}
