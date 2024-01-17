use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, StdError, StdResult, Uint128};
use elys_bindings::{
    query_resp::OracleAssetInfoResponse,
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
        let OracleAssetInfoResponse { asset_info } = querier.asset_info(coin.denom.clone())?;
        let decimal_point_coin = asset_info.decimal;
        let big_denom_unit = u64::checked_pow(10, decimal_point_coin as u32).unwrap();

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
        
        // if the amount is too small, we should use big denom amount instead in order to avoid crashing from amm module
        let coin_to_estimate = Coin{
            denom: coin.denom.clone(),
            amount: coin.amount.max(Uint128::from(big_denom_unit)),
        };
        
        let price = querier.get_amm_price_by_denom(coin_to_estimate, Decimal::zero())?;

        // TODO: Remove the commented functions below once the new query works.
        // let AmmSwapEstimationByDenomResponse {
        //     spot_price: price,
        //     amount: whole_value,
        //     ..
        // } = querier
        //     .get_amm_price_by_denom(coin_to_estimate, Decimal::zero())
        //     .map_err(|_e| StdError::generic_err("52"))?;

        let OracleAssetInfoResponse { asset_info } = querier.asset_info(value_denom.to_owned())?;

        let decimal_point_value = asset_info.decimal;
        let amount = Decimal::from_atomics(coin.amount, decimal_point_coin as u32)
            .map_err(|err| StdError::generic_err(err.to_string()))?;

        let value = price.checked_mul(Decimal::from_atomics(coin.amount, decimal_point_value as u32).unwrap())
            .map_err(|err| StdError::generic_err(err.to_string()))?;

        Ok(Self {
            denom: coin.denom.clone(),
            amount,
            price,
            value,
        })
    }
}
