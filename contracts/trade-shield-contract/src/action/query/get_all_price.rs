use super::*;
use crate::msg::query_resp::GetAllPricesResponse;

pub fn get_all_prices(
    deps: Deps<ElysQuery>,
    limit: u64,
) -> Result<GetAllPricesResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let mut pagination = PageRequest::new(limit);
    let prices: Vec<Price> = querier.oracle_get_all_prices(&mut pagination)?;

    let resp: GetAllPricesResponse = GetAllPricesResponse { prices };

    Ok(resp)
}
