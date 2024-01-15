use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, StdError, StdResult};
use elys_bindings::{
    query_resp::{AmmSwapEstimationByDenomResponse, OracleAssetInfoResponse},
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
        let OracleAssetInfoResponse { asset_info } = querier
            .asset_info(coin.denom.clone())
            .map_err(|e| StdError::generic_err("30"))?;
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
            .map_err(|e| StdError::generic_err("52"))?;

        let OracleAssetInfoResponse { asset_info } = querier
            .asset_info(value_denom.to_owned())
            .map_err(|e| StdError::generic_err(value_denom))?;

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
