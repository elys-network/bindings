use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, StdError, StdResult};
use elys_bindings::{
    query_resp::{AmmSwapEstimationByDenomResponse, OracleAssetInfoResponse},
    types::OracleAssetInfo,
    ElysQuerier,
};

#[cw_serde]
pub struct CoinValue {
    pub denom: String,
    pub amount: Decimal,
    pub price: Decimal,
    pub value: Decimal,
}

impl CoinValue {
    pub fn new(denom: String, amount: Decimal, price: Decimal, value: Decimal) -> Self {
        Self {
            denom,
            amount,
            price,
            value,
        }
    }
    pub fn from_coin(
        coin: &Coin,
        querier: &ElysQuerier<'_>,
        value_denom: &String,
    ) -> StdResult<Self> {
        let OracleAssetInfoResponse { asset_info } = match querier.asset_info(coin.denom.clone()) {
            Ok(res) => res,
            Err(_) => OracleAssetInfoResponse {
                asset_info: OracleAssetInfo {
                    denom: coin.denom.clone(),
                    display: coin.denom.clone(),
                    band_ticker: coin.denom.clone(),
                    elys_ticker: coin.denom.clone(),
                    decimal: 6,
                },
            },
        };
        let decimal_point_coin = asset_info.decimal;

        if &coin.denom == value_denom {
            let amount = Decimal::from_atomics(coin.amount, decimal_point_coin as u32)
                .map_err(|err| StdError::generic_err(err.to_string()))?;
            return Ok(Self {
                denom: coin.denom.clone(),
                value: amount.clone(),
                price: Decimal::one(),
                amount,
            });
        }

        let AmmSwapEstimationByDenomResponse {
            spot_price: price,
            amount: whole_value,
            ..
        } = querier
            .amm_swap_estimation_by_denom(&coin, &coin.denom, value_denom, &Decimal::zero())
            .map_err(|_e| StdError::generic_err("52"))?;

        // invert the price
        let price = Decimal::one() / price;

        let decimal_point_value = asset_info.decimal;
        let amount = Decimal::from_atomics(coin.amount, decimal_point_coin as u32)
            .map_err(|err| StdError::generic_err(err.to_string()))?;

        let value = Decimal::from_atomics(whole_value.amount, decimal_point_value as u32)
            .map_err(|err| StdError::generic_err(err.to_string()))?;

        Ok(Self {
            denom: coin.denom.clone(),
            amount,
            price,
            value,
        })
    }
}
