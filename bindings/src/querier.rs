use std::str::FromStr;

use cosmwasm_std::{
    coin, to_json_vec, Binary, Coin, ContractResult, Decimal, QuerierWrapper, QueryRequest,
    SignedDecimal, SignedDecimal256, StdError, StdResult, SystemResult,
};

use crate::{
    query::*,
    query_resp::*,
    trade_shield::types::StakedPosition,
    types::{BalanceAvailable, PageRequest, PerpetualPosition, Price, SwapAmountInRoute},
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
    pub fn mtp(&self, address: String, id: u64) -> StdResult<PerpetualMtpResponse> {
        let request = QueryRequest::Custom(ElysQuery::mtp(address, id));
        let resp: PerpetualMtpResponse = self.querier.query(&request)?;
        Ok(resp)
    }
    pub fn positions(&self, pagination: PageRequest) -> StdResult<PerpetualQueryPositionsResponse> {
        let request = QueryRequest::Custom(ElysQuery::positions(pagination));
        let resp: PerpetualQueryPositionsResponse = self.querier.query(&request)?;
        Ok(resp)
    }
    pub fn accounts(&self, pagination: Option<PageRequest>) -> StdResult<AuthAddressesResponse> {
        let request = QueryRequest::Custom(ElysQuery::accounts(pagination));

        let res: AuthAddressesResponse = self.querier.query(&request)?;

        Ok(res)
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

    pub fn perpetual_open_estimation(
        &self,
        position: PerpetualPosition,
        leverage: SignedDecimal,
        trading_asset: impl Into<String>,
        collateral: Coin,
        take_profit_price: Option<SignedDecimal256>,
        discount: Decimal,
    ) -> StdResult<PerpetualOpenEstimationResponse> {
        let query = ElysQuery::perpetual_open_estimation(
            position as i32,
            leverage,
            trading_asset.into(),
            collateral,
            take_profit_price,
            discount,
        );
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(query);

        let raw_resp: PerpetualOpenEstimationRawResponse = self.querier.query(&request)?;

        let resp: PerpetualOpenEstimationResponse = PerpetualOpenEstimationResponse {
            position: PerpetualPosition::try_from_i32(raw_resp.position)?,
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
            liquidation_price: SignedDecimal::from_str(&raw_resp.liquidation_price)
                .map_or(SignedDecimal::zero(), |liquidation_price| liquidation_price),
            estimated_pnl: raw_resp.estimated_pnl,
            estimated_pnl_denom: raw_resp.estimated_pnl_denom,
            available_liquidity: raw_resp.available_liquidity,
            weight_balance_ratio: SignedDecimal::from_str(&raw_resp.weight_balance_ratio)
                .map_or(SignedDecimal::zero(), |weight_balance_ratio| {
                    weight_balance_ratio
                }),
            borrow_interest_rate: SignedDecimal::from_str(&raw_resp.borrow_interest_rate)
                .map_or(SignedDecimal::zero(), |borrow_interest_rate| {
                    borrow_interest_rate
                }),
            funding_rate: SignedDecimal::from_str(&raw_resp.funding_rate)
                .map_or(SignedDecimal::zero(), |funding_rate| funding_rate),
            price_impact: SignedDecimal::from_str(&raw_resp.price_impact)
                .map_or(SignedDecimal::zero(), |price_impact| price_impact),
        };

        Ok(resp)
    }

    pub fn get_all_asset_profile(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryGetEntryAllResponse> {
        let all_asset_profile = ElysQuery::get_all_asset_profile(pagination);
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(all_asset_profile);
        let resp = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_asset_profile(&self, base_denom: String) -> StdResult<QueryGetEntryResponse> {
        let asset_profile = ElysQuery::get_asset_profile(base_denom.to_owned());
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(asset_profile);
        let QueryGetEntryResponseRaw { entry: raw_entry } = self.querier.query(&request)?;

        let resp = QueryGetEntryResponse {
            entry: Entry {
                base_denom: raw_entry
                    .base_denom
                    .map_or("".to_string(), |base_denom| base_denom),
                decimals: raw_entry.decimals.map_or(0, |decimals| decimals),
                denom: raw_entry.denom.map_or("".to_string(), |denom| denom),
                path: raw_entry.path.map_or("".to_string(), |path| path),
                ibc_channel_id: raw_entry
                    .ibc_channel_id
                    .map_or("".to_string(), |ibc_channel_id| ibc_channel_id),
                ibc_counterparty_channel_id: raw_entry
                    .ibc_counterparty_channel_id
                    .map_or("".to_string(), |ibc_counterparty_channel_id| {
                        ibc_counterparty_channel_id
                    }),
                display_name: raw_entry
                    .display_name
                    .map_or("".to_string(), |display_name| display_name),
                display_symbol: raw_entry
                    .display_symbol
                    .map_or("".to_string(), |display_symbol| display_symbol),
                external_symbol: raw_entry
                    .external_symbol
                    .map_or("".to_string(), |external_symbol| external_symbol),
                unit_denom: raw_entry
                    .unit_denom
                    .map_or("".to_string(), |unit_denom| unit_denom),
                authority: raw_entry
                    .authority
                    .map_or("".to_string(), |authority| authority),
                commit_enabled: raw_entry
                    .commit_enabled
                    .map_or(false, |commit_enabled| commit_enabled),
                withdraw_enabled: raw_entry
                    .withdraw_enabled
                    .map_or(false, |withdraw_enabled| withdraw_enabled),
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

    pub fn perpetual_get_position_for_address(
        &self,
        address: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> StdResult<PerpetualGetPositionsForAddressResponse> {
        let request = QueryRequest::Custom(ElysQuery::perpetual_get_position_for_address(
            address.into(),
            pagination,
        ));
        let raw_resp: PerpetualGetPositionsForAddressResponseRaw = self.querier.query(&request)?;

        let resp = PerpetualGetPositionsForAddressResponse {
            mtps: raw_resp.mtps.map_or(vec![], |mtps| mtps),
            pagination: raw_resp.pagination,
        };

        Ok(resp)
    }

    pub fn get_incentive_apr(&self, program: i32, denom: String) -> StdResult<QueryAprResponse> {
        let incentive_apr_query = ElysQuery::IncentiveApr {
            withdraw_type: program.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(incentive_apr_query);
        let resp: QueryAprResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_sub_bucket_rewards_balance(
        &self,
        address: String,
        denom: String,
        program: i32,
    ) -> StdResult<BalanceAvailable> {
        let sub_bucket_reward_query = ElysQuery::CommitmentRewardsSubBucketBalanceOfDenom {
            address,
            denom,
            program,
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(sub_bucket_reward_query);
        let resp: BalanceAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_oracle_price(
        &self,
        asset: String,
        source: String,
        timestamp: u64,
    ) -> StdResult<QueryGetPriceResponse> {
        let oracle_price_query = ElysQuery::OraclePrice {
            asset,
            source,
            timestamp,
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(oracle_price_query);
        let resp: QueryGetPriceResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_staked_balance(&self, address: String, denom: String) -> StdResult<StakedAvailable> {
        let staked_balance_query = ElysQuery::CommitmentStakedBalanceOfDenom {
            address: address.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(staked_balance_query);
        let resp: StakedAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_amm_price_by_denom(&self, token_in: Coin, discount: Decimal) -> StdResult<Decimal> {
        let amm_price_query = ElysQuery::AmmPriceByDenom {
            token_in: token_in.to_owned(),
            discount: discount.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(amm_price_query);
        let resp: Decimal = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_staked_positions(&self, address: String) -> StdResult<QueryStakedPositionResponse> {
        let staked_position_query = ElysQuery::CommitmentStakedPositions {
            delegator_address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(staked_position_query);
        let resp: QueryStakedPositionResponse = self.querier.query(&request)?;
        if resp.staked_position.is_none() {
            return Ok(resp);
        }

        let stacks: Vec<StakedPosition> = resp
            .staked_position
            .unwrap()
            .iter()
            .filter(|stack| !(stack.staked.amount.is_zero()))
            .cloned()
            .collect();

        let resp = QueryStakedPositionResponse {
            staked_position: Some(stacks),
        };

        Ok(resp)
    }

    pub fn get_unstaked_positions(
        &self,
        address: String,
    ) -> StdResult<QueryUnstakedPositionResponse> {
        let unstaked_position_query = ElysQuery::CommitmentUnStakedPositions {
            delegator_address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(unstaked_position_query);
        let resp: QueryUnstakedPositionResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_borrowed_balance(&self) -> StdResult<BalanceBorrowed> {
        let borrowed_balance_query = ElysQuery::StableStakeBalanceOfBorrow {};
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(borrowed_balance_query);
        let resp: BalanceBorrowed = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_stable_stake_params(&self) -> StdResult<StableStakeParamsData> {
        let query: ElysQuery = ElysQuery::StableStakeParams {};
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(query);
        let resp: StableStakeParamsResp = self.querier.query(&request)?;
        Ok(resp.params)
    }

    pub fn get_delegations(
        &self,
        delegator_addr: String,
    ) -> StdResult<QueryDelegatorDelegationsResponse> {
        let delegations_query = ElysQuery::CommitmentDelegations {
            delegator_address: delegator_addr.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(delegations_query);
        let resp: QueryDelegatorDelegationsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_unbonding_delegations(
        &self,
        delegator_addr: String,
    ) -> StdResult<QueryDelegatorUnbondingDelegationsResponse> {
        let unbonding_delegations_query = ElysQuery::CommitmentUnbondingDelegations {
            delegator_address: delegator_addr.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(unbonding_delegations_query);
        let resp: QueryDelegatorUnbondingDelegationsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_all_validators(
        &self,
        delegator: String,
    ) -> StdResult<QueryDelegatorValidatorsResponse> {
        let validators_query = ElysQuery::CommitmentAllValidators {
            delegator_address: delegator.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(validators_query);
        let resp: QueryDelegatorValidatorsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_delegator_validators(
        &self,
        delegator: String,
    ) -> StdResult<QueryDelegatorValidatorsResponse> {
        let validators_query = ElysQuery::CommitmentDelegatorValidators {
            delegator_address: delegator.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(validators_query);
        let resp: QueryDelegatorValidatorsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_commitments(&self, address: String) -> StdResult<QueryShowCommitmentsResponse> {
        let commitments_query = ElysQuery::CommitmentShowCommitments {
            creator: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(commitments_query);
        let resp: QueryShowCommitmentsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_rewards_balance(
        &self,
        address: String,
        denom: String,
    ) -> StdResult<BalanceAvailable> {
        let rewards_balance_query = ElysQuery::CommitmentRewardsBalanceOfDenom {
            address: address.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(rewards_balance_query);
        let resp: BalanceAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_vesting_info(&self, address: String) -> StdResult<QueryVestingInfoResponse> {
        let vesting_info_query = ElysQuery::CommitmentVestingInfo {
            address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(vesting_info_query);
        let resp: QueryVestingInfoResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_all_pools(
        &self,
        pool_ids: Option<Vec<u64>>,
        filter_type: i32,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryEarnPoolResponse> {
        let pools_query = ElysQuery::get_all_pools(pool_ids, filter_type, pagination);
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(pools_query);

        let resp: QueryEarnPoolResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_asset_price(&self, asset: impl Into<String>) -> StdResult<Decimal> {
        let asset: String = asset.into();

        let QueryGetEntryResponse {
            entry: Entry {
                denom: usdc_denom, ..
            },
        } = self.get_asset_profile("uusdc".to_string())?;

        if asset == usdc_denom {
            return Ok(Decimal::one());
        }

        // FIXME: convert first 1USDC to DENOM IN and use the result as input amount to convert DENOM IN to DENOM OUT

        let spot_price = self
            .get_amm_price_by_denom(coin(1000000, asset), Decimal::one())
            .map_err(|e| {
                StdError::generic_err(format!("get_asset_price: spot price not found:{:?}", e))
            })?;

        Ok(spot_price)
    }

    pub fn get_asset_price_from_denom_in_to_denom_out(
        &self,
        denom_in: impl Into<String>,
        denom_out: impl Into<String>,
    ) -> StdResult<Decimal> {
        let price_in = self.get_asset_price(denom_in)?;
        let price_out = self.get_asset_price(denom_out)?;

        price_in.checked_div(price_out).map_err(|e| {
            StdError::generic_err(format!(
                "get_asset_price_from_denom_in_to_denom_out: price calculation error : {:?}",
                e
            ))
        })
    }
    #[allow(dead_code)]
    #[cfg(feature = "debug")]
    fn query_binary(&self, request: &QueryRequest<ElysQuery>) -> StdResult<Binary> {
        let raw = to_json_vec(request).map_err(|serialize_err| {
            StdError::generic_err(format!("Serializing QueryRequest: {serialize_err}"))
        })?;
        match self.querier.raw_query(&raw) {
            SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
                "Querier system error: {system_err}"
            ))),
            SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(
                format!("Querier contract error: {contract_err}"),
            )),
            SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
        }
    }
}
