use cosmwasm_std::{coin, Coin, QuerierWrapper, QueryRequest, StdResult};

use crate::{
    query::*,
    query_resp::*,
    types::{PageRequest, SwapAmountInRoute},
};

pub struct ElysQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ElysQuery>,
}

impl<'a> ElysQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, ElysQuery>) -> Self {
        ElysQuerier { querier }
    }
    pub fn oracle_get_all_prices(&self, pagination: &mut PageRequest) -> StdResult<Vec<Coin>> {
        let prices_query = ElysQuery::oracle_get_all_prices(pagination.clone());
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(prices_query);

        let resp: OraclePriceAllResponse = self.querier.query(&request)?;

        pagination.update(resp.pagination.next_key);
        let result: Vec<Coin> = resp
            .price
            .iter()
            .map(|price| coin(price.price.atomics().u128(), &price.asset))
            .collect();
        Ok(result)
    }
    pub fn amm_swap_estimation(
        &self,
        routes: &Vec<SwapAmountInRoute>,
        token_in: &Coin,
    ) -> StdResult<AmmSwapEstimationResponse> {
        let request = QueryRequest::Custom(ElysQuery::amm_swap_estimation(
            routes.to_owned(),
            token_in.to_owned(),
        ));
        let resp: AmmSwapEstimationResponse = self.querier.query(&request)?;
        Ok(resp)
    }
    pub fn asset_info(&self, denom: String) -> StdResult<OracleAssetInfoResponse> {
        let request = QueryRequest::Custom(ElysQuery::asset_info(denom));
        let resp: OracleAssetInfoResponse = self.querier.query(&request)?;
        Ok(resp)
    }
}
