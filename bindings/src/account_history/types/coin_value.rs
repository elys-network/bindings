use std::str::FromStr;

use crate::{query_resp::OracleAssetInfoResponse, types::OracleAssetInfo, ElysQuerier};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, DecCoin, Decimal, Decimal256, StdError, StdResult, Uint256};

#[cw_serde]
#[derive(Default)]
pub struct CoinValue {
    pub denom: String,
    pub amount_token: Decimal,
    pub price: Decimal,
    pub amount_usd: Decimal,
}

#[cw_serde]
#[derive(Default)]
pub struct Coin256Value {
    pub denom: String,
    pub amount_token: Decimal256,
    pub price: Decimal,
    pub amount_usd: Decimal256,
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

        let price = querier
            .get_asset_price(balance.denom.clone())
            .map_err(|e| StdError::generic_err(format!("failed to get_asset_price: {}", e)))?;

        let decimal_point_usd = asset_info.decimal;

        let amount_token = Decimal::from_atomics(balance.amount, decimal_point_usd as u32)
            .map_err(|e| {
                StdError::generic_err(format!("failed to convert amount to Decimal: {}", e))
            })?;

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

impl Coin256Value {
    pub fn new(
        denom: String,
        amount_token: Decimal256,
        price: Decimal,
        amount_usd: Decimal256,
    ) -> Self {
        Self {
            denom,
            amount_token,
            amount_usd,
            price,
        }
    }

    pub fn from_coin256(balance: &Coin256, querier: &ElysQuerier<'_>) -> StdResult<Self> {
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
        let decimal_point_token = asset_info.decimal;
        let price = querier
            .get_asset_price(balance.denom.clone())
            .map_err(|e| StdError::generic_err(format!("failed to get_asset_price: {}", e)))?;

        let amount_token = Decimal256::from_atomics(balance.amount, decimal_point_token as u32)
            .map_err(|e| {
                StdError::generic_err(format!("failed to convert amount to Decimal: {}", e))
            })?;

        let amount_usd = amount_token
            .checked_mul(Decimal256::from(price.clone()))
            .map_err(|e| {
                StdError::generic_err(format!(
                    "failed to convert amount_usd_base to Decimal: {}",
                    e
                ))
            })?;

        Ok(Self {
            denom: balance.denom.clone(),
            amount_token: Decimal256::from_str(balance.amount.to_string().as_str())?,
            price,
            amount_usd,
        })
    }
}
#[cw_serde]
#[derive(Default)]
pub struct Coin256 {
    pub denom: String,
    pub amount: Uint256,
}

impl From<DecCoin> for Coin256 {
    fn from(value: DecCoin) -> Self {
        Coin256 {
            denom: value.denom,
            amount: value.amount.atomics(),
        }
    }
}
