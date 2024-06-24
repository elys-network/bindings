use std::collections::HashMap;

use crate::account_history::states::PRICE_CACHE;
use crate::{query_resp::OracleAssetInfoResponse, types::OracleAssetInfo, ElysQuerier};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Env, StdError, StdResult, Storage};

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

    pub fn update_from_coin(
        balance: &Coin,
        querier: &ElysQuerier<'_>,
        env: Env,
        store: &mut dyn Storage,
    ) -> StdResult<Self> {
        let price_cache = PRICE_CACHE.may_load(store, env.block.height)?;

        if price_cache.is_none() {
            let (coin_value, decimal) = Self::from_coin_int(balance, querier)?;
            let mut value = HashMap::new();
            value.insert(balance.denom.clone(), (coin_value.amount_usd, decimal));
            PRICE_CACHE.save(store, env.block.height, &value)?;
            return Ok(coin_value);
        }

        if let Some(price_and_denom) = price_cache.unwrap().get(&balance.denom) {
            let (amount_token, amount_usd) =
                Self::calculate(price_and_denom.1, balance, price_and_denom.0)?;

            return Ok(Self {
                denom: balance.denom.clone(),
                amount_token,
                price: price_and_denom.0,
                amount_usd,
            });
        }
        let (coin_value, decimal) = Self::from_coin_int(balance, querier)?;
        let mut value = HashMap::new();
        value.insert(balance.denom.clone(), (coin_value.amount_usd, decimal));
        PRICE_CACHE.save(store, env.block.height, &value)?;

        return Ok(coin_value);
    }

    pub fn from_coin(balance: &Coin, querier: &ElysQuerier<'_>) -> StdResult<Self> {
        let coin_value = Self::from_coin_int(balance, querier)?;

        Ok(coin_value.0)
    }

    fn from_coin_int(balance: &Coin, querier: &ElysQuerier<'_>) -> StdResult<(Self, u64)> {
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

        let mut price = Decimal::zero();
        if balance.denom != ElysDenom::EdenBoost.as_str() {
            price = querier
                .get_asset_price(balance.denom.clone())
                .map_err(|e| StdError::generic_err(format!("failed to get_asset_price: {}", e)))?;
        }

        let (amount_token, amount_usd) = Self::calculate(decimal_point_usd, balance, price)?;

        Ok((
            Self {
                denom: balance.denom.clone(),
                amount_token,
                price,
                amount_usd,
            },
            decimal_point_usd,
        ))
    }

    fn calculate(
        decimal_point_usd: u64,
        balance: &Coin,
        price: Decimal,
    ) -> StdResult<(Decimal, Decimal)> {
        let amount_token = Decimal::from_atomics(balance.amount, decimal_point_usd as u32)
            .map_err(|e| {
                StdError::generic_err(format!("failed to convert amount to Decimal: {}", e))
            })?;
        if balance.denom == ElysDenom::EdenBoost.as_str() {
            return Ok((amount_token, Decimal::zero()));
        }

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

        Ok((amount_token, amount_usd))
    }
}
