use cosmwasm_std::{
    coin, from_json, to_json_vec, Binary, Coin, ContractResult, QuerierWrapper, QueryRequest,
    StdError, StdResult, SystemResult,
};

use crate::{
    query::*,
    query_resp::*,
    types::{PageRequest, Price, SwapAmountInRoute},
};

pub struct ElysQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ElysQuery>,
}

impl<'a> ElysQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, ElysQuery>) -> Self {
        ElysQuerier { querier }
    }
    pub fn oracle_get_all_prices(&self, pagination: &mut PageRequest) -> StdResult<Vec<Price>> {
        let prices_query = ElysQuery::oracle_get_all_prices(pagination.clone());
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(prices_query);

        let resp: OracleAllPriceResponse = self.querier.query(&request)?;

        pagination.update(resp.pagination.next_key);

        let prices = match resp.price {
            Some(prices) => prices,
            None => vec![],
        };

        Ok(prices)
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
        let request = QueryRequest::Custom(ElysQuery::oracle_asset_info(denom));
        let resp: OracleAssetInfoResponse = self.querier.query(&request)?;
        Ok(resp)
    }
}
