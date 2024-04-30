use crate::{query_resp::OracleAssetInfoResponse, types::OracleAssetInfo, ElysQuerier};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, DecCoin, Decimal, Decimal256, StdError, StdResult};

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
pub struct DecCoinValue {
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

    pub fn from_coin(
        balance: &Coin,
        querier: &ElysQuerier<'_>,
        usdc_denom: &String,
    ) -> StdResult<Self> {
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

        if &balance.denom == usdc_denom {
            let amount = Decimal::from_atomics(balance.amount, decimal_point_token as u32)
                .map_err(|e| {
                    StdError::generic_err(format!("failed to convert amount to Decimal: {}", e))
                })?;
            let price = querier.get_asset_price(usdc_denom)?;
            let amount_usd = amount.checked_mul(price.clone()).map_err(|e| {
                StdError::generic_err(format!("failed to convert amount to amount_usd: {}", e))
            })?;
            return Ok(Self {
                denom: balance.denom.clone(),
                amount_usd,
                price,
                amount_token: amount,
            });
        }

        let price = querier
            .get_asset_price(balance.denom.clone())
            .map_err(|e| StdError::generic_err(format!("failed to get_asset_price: {}", e)))?;

        let decimal_point_usd = asset_info.decimal;

        let amount_token = Decimal::from_atomics(balance.amount, decimal_point_token as u32)
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

impl DecCoinValue {
    pub fn from_dec_coin(
        balance: &DecCoin,
        querier: &ElysQuerier<'_>,
        usdc_denom: &String,
    ) -> StdResult<Self> {
        if &balance.denom == usdc_denom {
            let price = querier.get_asset_price(usdc_denom)?;
            let amount_usd = balance
                .amount
                .checked_mul(Decimal256::from(price.clone()))
                .map_err(|e| {
                    StdError::generic_err(format!("failed to convert amount to amount_usd: {}", e))
                })?;

            return Ok(Self {
                denom: balance.denom.clone(),
                amount_usd,
                price,
                amount_token: balance.amount,
            });
        }

        let price = querier
            .get_asset_price(balance.denom.clone())
            .map_err(|e| StdError::generic_err(format!("failed to get_asset_price: {}", e)))?;

        let amount_usd = balance
            .amount
            .checked_mul(Decimal256::from(price.clone()))
            .map_err(|e| {
                StdError::generic_err(format!(
                    "failed to convert amount_usd_base to Decimal: {}",
                    e
                ))
            })?;

        Ok(Self {
            denom: balance.denom.clone(),
            amount_token: balance.amount.clone(),
            price,
            amount_usd,
        })
    }
}