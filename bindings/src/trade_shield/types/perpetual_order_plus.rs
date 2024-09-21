use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, SignedDecimal, StdError, StdResult};

use super::{OrderPrice, PerpetualOrderV2};

#[cw_serde]
pub struct PerpetualOrderPlus {
    pub order: PerpetualOrderV2,
    pub order_price: Option<Decimal>,
    pub custody: Coin,
}

impl PerpetualOrderPlus {
    // custody = collateral * leverage / trigger_price
    fn calculate_custody(
        price: &OrderPrice,
        leverage: &SignedDecimal,
        collateral: &Coin,
        trading_asset: &str,
    ) -> StdResult<Coin> {
        let leverage_decimal =
            Decimal::from_atomics(leverage.atomics().i128() as u128, leverage.decimal_places())
                .map_err(|e| {
                    StdError::generic_err(format!("failed to convert leverage to Decimal: {}", e))
                })?;
        let rate_decimal = price.rate;
        let collateral_decimal =
            Decimal::from_atomics(collateral.amount.u128(), 0).map_err(|e| {
                StdError::generic_err(format!("failed to convert collateral to Decimal: {}", e))
            })?;

        // Perform the calculation
        let custody_value = collateral_decimal
            .checked_mul(leverage_decimal)?
            .checked_div(rate_decimal)
            .map_err(|e| StdError::generic_err(format!("failed to calculate custody: {}", e)))?;

        Ok(coin(custody_value.to_uint_floor().u128(), trading_asset))
    }

    pub fn new(order: PerpetualOrderV2) -> StdResult<Self> {
        if order.trigger_price.is_none() {
            return Ok(Self {
                custody: coin(0, &order.trading_asset),
                order,
                order_price: None,
            });
        }

        let order_price = order.trigger_price.clone().unwrap();
        let custody = Self::calculate_custody(
            &order_price,
            &order.leverage,
            &order.collateral,
            &order.trading_asset,
        )?;

        Ok(Self {
            order_price: Some(order_price.rate),
            custody,
            order,
        })
    }
}
