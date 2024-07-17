use crate::helper::get_discount;

use super::*;
use cosmwasm_std::{Coin, Decimal, SignedDecimal, SignedDecimal256, StdResult};
use elys_bindings::query_resp::PerpetualOpenEstimationResponse;

pub fn perpetual_open_estimation(
    deps: Deps<ElysQuery>,
    position: PerpetualPosition,
    leverage: SignedDecimal,
    trading_asset: String,
    collateral: Coin,
    take_profit_price: Option<SignedDecimal256>,
    user_address: Option<String>, // Parameter unused until account history work
) -> StdResult<PerpetualOpenEstimationResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    let discount = match user_address {
        Some(user_address) => get_discount(deps.querier, user_address)?,
        None => Decimal::zero(),
    };

    querier.perpetual_open_estimation(
        position,
        leverage,
        trading_asset,
        collateral,
        take_profit_price,
        discount,
    )
}
