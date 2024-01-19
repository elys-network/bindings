use cosmwasm_std::{Coin, Decimal, Deps, StdResult};
use elys_bindings::{query_resp::AmmSwapEstimationByDenomResponse, ElysQuerier, ElysQuery};

pub fn swap_estimation_by_denom(
    deps: Deps<ElysQuery>,
    amount: Coin,
    denom_in: String,
    denom_out: String,
    _user_address: Option<String>, // Parameter unused until account history work
) -> StdResult<AmmSwapEstimationByDenomResponse> {
    let querier = ElysQuerier::new(&deps.querier);

    let resp: AmmSwapEstimationByDenomResponse =
        querier.amm_swap_estimation_by_denom(&amount, denom_in, denom_out, &Decimal::zero())?;

    Ok(resp)
}
