use super::*;
use cosmwasm_std::{Coin, Decimal, SignedDecimal, SignedDecimal256, StdResult};
use elys_bindings::query_resp::MarginOpenEstimationResponse;

pub fn margin_open_estimation(
    deps: Deps<ElysQuery>,
    position: MarginPosition,
    leverage: SignedDecimal,
    trading_asset: String,
    collateral: Coin,
    take_profit_price: Option<SignedDecimal256>,
    _user_address: Option<String>, // Parameter unused until account history work
) -> StdResult<MarginOpenEstimationResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    querier.margin_open_estimation(
        position,
        leverage,
        trading_asset,
        collateral,
        take_profit_price,
        Decimal::zero(),
    )
}
