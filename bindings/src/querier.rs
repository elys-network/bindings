use cosmwasm_std::{Coin, Decimal, QuerierWrapper, QueryRequest, StdResult};

use crate::{
    query::*,
    query_resp::*,
    types::{BalanceAvailable, MarginPosition, PageRequest, Price, SwapAmountInRoute},
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
        discount: &Decimal,
    ) -> StdResult<AmmSwapEstimationResponse> {
        let request = QueryRequest::Custom(ElysQuery::amm_swap_estimation(
            routes.to_owned(),
            token_in.to_owned(),
            discount.to_owned(),
        ));
        let resp: AmmSwapEstimationResponse = self.querier.query(&request)?;
        Ok(resp)
    }
    pub fn amm_swap_estimation_by_denom(
        &self,
        amount: &Coin,
        denom_in: impl Into<String>,
        denom_out: impl Into<String>,
        discount: &Decimal,
    ) -> StdResult<AmmSwapEstimationByDenomResponse> {
        let request = QueryRequest::Custom(ElysQuery::amm_swap_estimation_by_denom(
            amount.to_owned(),
            denom_in.into(),
            denom_out.into(),
            discount.to_owned(),
        ));
        let resp: AmmSwapEstimationByDenomResponse = self.querier.query(&request)?;

        Ok(resp)
    }
    pub fn asset_info(&self, denom: String) -> StdResult<OracleAssetInfoResponse> {
        let request = QueryRequest::Custom(ElysQuery::oracle_asset_info(denom));
        let resp: OracleAssetInfoResponse = self.querier.query(&request)?;
        Ok(resp)
    }
    pub fn mtp(&self, address: String, id: u64) -> StdResult<MarginMtpResponse> {
        let request = QueryRequest::Custom(ElysQuery::mtp(address, id));
        let resp: MarginMtpResponse = self.querier.query(&request)?;
        Ok(resp)
    }
    pub fn positions(&self, pagination: PageRequest) -> StdResult<MarginQueryPositionsResponse> {
        let request = QueryRequest::Custom(ElysQuery::positions(pagination));
        let resp: MarginQueryPositionsResponse = self.querier.query(&request)?;
        Ok(resp)
    }
    pub fn accounts(&self, pagination: PageRequest) -> StdResult<AuthAccountsResponse> {
        let request = QueryRequest::Custom(ElysQuery::accounts(pagination));
        let resp: AuthAccountsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_balance(&self, address: String, denom: String) -> StdResult<BalanceAvailable> {
        let balance_query = ElysQuery::AmmBalance {
            address: address.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(balance_query);
        let resp: BalanceAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn margin_open_estimation(
        &self,
        position: MarginPosition,
        leverage: Decimal,
        trading_asset: impl Into<String>,
        collateral: Coin,
        take_profit_price: Decimal,
        discount: Decimal,
    ) -> StdResult<MarginOpenEstimationResponse> {
        let query = ElysQuery::margin_open_estimation(
            position as i32,
            leverage,
            trading_asset.into(),
            collateral,
            take_profit_price,
            discount,
        );
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(query);

        let resp: MarginOpenEstimationResponse = self.querier.query(&request)?;
        Ok(resp)
    }
    
    pub fn get_asset_profile(&self, base_denom: String ) -> StdResult<QueryGetEntryResponse> {
        let asset_profile = ElysQuery::get_asset_profile(base_denom.to_owned());
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(asset_profile);
        let resp: QueryGetEntryResponse = self.querier.query(&request)?;
        Ok(resp)
    }
}
