use crate::{query_resp::OracleAssetInfoResponse, types::OracleAssetInfo, ElysQuerier};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, StdError, StdResult};

#[cw_serde]
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
    pub fn from_coin(
        balance: &Coin,
        querier: &ElysQuerier<'_>,
        usdc_denom: &String,
    ) -> StdResult<Self> {
        let OracleAssetInfoResponse { asset_info } = match querier.asset_info(balance.denom.clone())
        {
            Ok(res) => res,
            Err(_) => OracleAssetInfoResponse {
                asset_info: OracleAssetInfo {
                    denom: balance.denom.clone(),
                    display: balance.denom.clone(),
                    band_ticker: balance.denom.clone(),
                    elys_ticker: balance.denom.clone(),
                    decimal: 6,
                },
            },
        };
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

        let asset_price_denom = if balance.denom == "ueden" {
            "uelys".to_string()
        } else {
            balance.denom.clone()
        };

        let price = querier
            .get_asset_price(asset_price_denom)
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
