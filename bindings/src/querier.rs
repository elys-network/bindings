use std::str::FromStr;

use cosmwasm_std::{
    Coin, Decimal, QuerierWrapper, QueryRequest, SignedDecimal, SignedDecimal256, StdResult,
};

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
        leverage: SignedDecimal,
        trading_asset: impl Into<String>,
        collateral: Coin,
        take_profit_price: SignedDecimal256,
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

        let raw_resp: MarginOpenEstimationRawResponse = self.querier.query(&request)?;

        let resp: MarginOpenEstimationResponse = MarginOpenEstimationResponse {
            position: raw_resp.position,
            leverage: SignedDecimal::from_str(&raw_resp.leverage)
                .map_or(SignedDecimal::zero(), |leverage| leverage),
            trading_asset: raw_resp.trading_asset,
            collateral: raw_resp.collateral,
            min_collateral: raw_resp.min_collateral,
            valid_collateral: raw_resp
                .valid_collateral
                .map_or(false, |valid_collateral| valid_collateral),
            position_size: raw_resp.position_size,
            swap_fee: Decimal::from_str(&raw_resp.swap_fee)
                .map_or(Decimal::zero(), |swap_fee| swap_fee),
            discount: Decimal::from_str(&raw_resp.discount)
                .map_or(Decimal::zero(), |discount| discount),
            open_price: Decimal::from_str(&raw_resp.open_price)
                .map_or(Decimal::zero(), |open_price| open_price),
            take_profit_price: SignedDecimal256::from_str(&raw_resp.take_profit_price)
                .map_or(SignedDecimal256::zero(), |take_profit_price| {
                    take_profit_price
                }),
            liquidation_price: Decimal::from_str(&raw_resp.liquidation_price)
                .map_or(Decimal::zero(), |liquidation_price| liquidation_price),
            estimated_pnl: raw_resp.estimated_pnl,
            available_liquidity: raw_resp.available_liquidity,
        };

        Ok(resp)
    }

    pub fn get_asset_profile(&self, base_denom: String) -> StdResult<QueryGetEntryResponse> {
        let asset_profile = ElysQuery::get_asset_profile(base_denom.to_owned());
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(asset_profile);
        let QueryGetEntryResponseRaw { entry: raw_entry }: QueryGetEntryResponseRaw =
            self.querier.query(&request)?;

        let resp = QueryGetEntryResponse {
            entry: Entry {
                base_denom: raw_entry.base_denom,
                decimals: raw_entry.decimals,
                denom: raw_entry.denom,
                path: raw_entry.path.map_or("".to_string(), |path| path),
                ibc_channel_id: raw_entry
                    .ibc_channel_id
                    .map_or("".to_string(), |ibc_channel_id| ibc_channel_id),
                ibc_counterparty_channel_id: raw_entry
                    .ibc_counterparty_channel_id
                    .map_or("".to_string(), |ibc_counterparty_channel_id| {
                        ibc_counterparty_channel_id
                    }),
                display_name: raw_entry.display_name,
                display_symbol: raw_entry
                    .display_symbol
                    .map_or("".to_string(), |display_symbol| display_symbol),
                external_symbol: raw_entry
                    .external_symbol
                    .map_or("".to_string(), |external_symbol| external_symbol),
                unit_denom: raw_entry
                    .unit_denom
                    .map_or("".to_string(), |unit_denom| unit_denom),
                authority: raw_entry.authority,
                commit_enabled: raw_entry.commit_enabled,
                withdraw_enabled: raw_entry.withdraw_enabled,
                network: raw_entry.network.map_or("".to_string(), |network| network),
                address: raw_entry.address.map_or("".to_string(), |address| address),
                transfer_limit: raw_entry
                    .transfer_limit
                    .map_or("".to_string(), |transfer_limit| transfer_limit),
                ibc_counterparty_denom: raw_entry
                    .ibc_counterparty_denom
                    .map_or("".to_string(), |ibc_counterparty_denom| {
                        ibc_counterparty_denom
                    }),
                ibc_counterparty_chain_id: raw_entry
                    .ibc_counterparty_chain_id
                    .map_or("".to_string(), |ibc_counterparty_chain_id| {
                        ibc_counterparty_chain_id
                    }),
                permissions: raw_entry
                    .permissions
                    .map_or(vec![], |permissions| permissions),
            },
        };
        Ok(resp)
    }

    pub fn margin_get_position_for_address(
        &self,
        address: impl Into<String>,
        pagination: PageRequest,
    ) -> StdResult<MarginGetPositionsForAddressResponse> {
        let request = QueryRequest::Custom(ElysQuery::margin_get_position_for_address(
            address.into(),
            pagination,
        ));
        let raw_resp: MarginGetPositionsForAddressResponseRaw = self.querier.query(&request)?;

        let resp = MarginGetPositionsForAddressResponse {
            mtps: raw_resp.mtps.map_or(vec![], |mtps| mtps),
            pagination: raw_resp.pagination,
        };

        Ok(resp)
    }
}
