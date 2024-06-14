use std::collections::HashMap;
use std::str::FromStr;

use cosmwasm_std::{
    coin, to_json_vec, Binary, Coin, ContractResult, Decimal, Int128, QuerierWrapper, QueryRequest,
    SignedDecimal, SignedDecimal256, StdError, StdResult, SystemResult, Uint128,
};

use crate::{
    account_history::types::CoinValue,
    query::*,
    query_resp::*,
    trade_shield::types::{
        AmmPool, PoolAsset, PoolExtraInfo, StakedPosition, StakedPositionRaw, StakingValidator,
    },
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
    pub fn amm_get_pool(&self, pool_id: u64) -> StdResult<AmmGetPoolResponse> {
        let request = QueryRequest::Custom(ElysQuery::amm_get_pool(pool_id));
        let raw_resp: AmmGetPoolResponseRaw = self.querier.query(&request)?;
        let resp = AmmGetPoolResponse {
            pool: raw_resp.pool.into(),
            extra_info: raw_resp.extra_info.unwrap_or(PoolExtraInfo {
                tvl: Decimal::zero(),
                lp_token_price: Decimal::zero(),
            }),
        };
        Ok(resp)
    }

    pub fn amm_get_pools(&self, pagination: Option<PageRequest>) -> StdResult<AmmGetPoolsResponse> {
        let request = QueryRequest::Custom(ElysQuery::amm_get_pools(pagination));
        let raw_resp: AmmGetPoolsResponseRaw = self.querier.query(&request)?;
        let pool: Vec<AmmPool> = raw_resp.pool.map_or(vec![], |pools| {
            pools
                .iter()
                .map(|pool| pool.to_owned().into())
                .collect::<Vec<AmmPool>>()
        });
        let extra_infos: Vec<PoolExtraInfo> = raw_resp.extra_infos.unwrap_or(vec![]);
        let resp = AmmGetPoolsResponse {
            pool,
            extra_infos,
            pagination: raw_resp.pagination,
        };
        Ok(resp)
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
            slippage: Decimal::from_str(&raw_resp.slippage)
                .map_or(Decimal::zero(), |slippage| slippage),
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

    pub fn get_incentive_aprs(&self) -> StdResult<QueryAprsResponse> {
        let request = QueryRequest::Custom(ElysQuery::get_incentive_aprs());
        let resp: QueryAprsResponse = self.querier.query(&request)?;
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

        let resp: QueryStakedPositionResponseRaw = self.querier.query(&request)?;
        if resp.staked_position.is_none() {
            return Ok(QueryStakedPositionResponse {
                staked_position: None,
            });
        }

        let stacks: Vec<StakedPositionRaw> = resp.staked_position.unwrap();

        let stacks: Vec<StakedPosition> = stacks
            .iter()
            .map(|stack| StakedPosition {
                id: stack.id.clone(),
                validator: StakingValidator {
                    id: Some(stack.validator.id.clone().map_or("".to_string(), |id| id)),
                    address: stack
                        .validator
                        .address
                        .clone()
                        .map_or("".to_string(), |address| address),
                    name: stack
                        .validator
                        .name
                        .clone()
                        .map_or("".to_string(), |name| name),
                    voting_power: stack
                        .validator
                        .voting_power
                        .clone()
                        .map_or(Decimal::zero(), |voting_power| voting_power),
                    commission: stack
                        .validator
                        .commission
                        .clone()
                        .map_or(Decimal::zero(), |commission| commission),
                },
                staked: stack.staked.clone(),
            })
            .filter(|stack| !(stack.staked.amount.is_zero()))
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
        let raw_resp: QueryShowCommitmentsResponseRaw = self.querier.query(&request)?;
        let vesting_tokens: Option<Vec<VestingTokens>> = match raw_resp.commitments.vesting_tokens {
            Some(vec) => Some(
                vec.iter()
                    .map(|vesting_token| VestingTokens {
                        denom: vesting_token.denom.clone(),
                        total_amount: vesting_token.total_amount.clone(),
                        claimed_amount: vesting_token.claimed_amount.clone(),
                        num_blocks: vesting_token.num_blocks.clone().unwrap_or(0),
                        start_block: vesting_token.start_block.clone().unwrap_or(0),
                        vest_started_timestamp: vesting_token
                            .vest_started_timestamp
                            .clone()
                            .unwrap_or(0),
                    })
                    .collect(),
            ),
            None => None,
        };
        let resp = QueryShowCommitmentsResponse {
            commitments: Commitments {
                creator: raw_resp.commitments.creator,
                committed_tokens: raw_resp.commitments.committed_tokens,
                rewards_unclaimed: raw_resp.commitments.rewards_unclaimed,
                claimed: raw_resp.commitments.claimed,
                vesting_tokens,
                rewards_by_elys_unclaimed: raw_resp.commitments.rewards_by_elys_unclaimed,
                rewards_by_eden_unclaimed: raw_resp.commitments.rewards_by_eden_unclaimed,
                rewards_by_edenb_unclaimed: raw_resp.commitments.rewards_by_edenb_unclaimed,
                rewards_by_usdc_unclaimed: raw_resp.commitments.rewards_by_usdc_unclaimed,
            },
        };

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

    pub fn join_pool_estimation(
        &self,
        pool_id: u64,
        amounts_in: Vec<Coin>,
    ) -> StdResult<QueryJoinPoolEstimationResponse> {
        let query = ElysQuery::join_pool_estimation(pool_id, amounts_in);
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(query);
        let response: QueryJoinPoolEstimationResponse = self.querier.query(&request)?;
        Ok(response)
    }

    pub fn exit_pool_estimation(
        &self,
        pool_id: u64,
        share_amount_in: Uint128,
    ) -> StdResult<QueryExitPoolEstimationResponse> {
        let query = ElysQuery::exit_pool_estimation(pool_id, share_amount_in, "".to_string());
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(query);
        let response: QueryExitPoolEstimationResponse = self.querier.query(&request)?;
        Ok(response)
    }

    pub fn get_current_pool_ratio(&self, pool: &PoolResp) -> HashMap<String, Decimal> {
        let mut current_ratio: HashMap<String, Decimal> = HashMap::new();

        let total_value: Decimal = pool.assets.iter().fold(Decimal::zero(), |acc, x| {
            if let Some(usd_value) = x.usd_value {
                return acc + usd_value;
            }
            return Decimal::zero();
        });

        // Calculate ratio for each asset in the pool
        for asset in &pool.assets {
            let ratio = asset.usd_value.map_or(Decimal::zero(), |usd_value| {
                usd_value
                    .checked_div(total_value)
                    .unwrap_or(Decimal::zero())
            });
            current_ratio.insert(asset.token.denom.clone(), ratio);
        }

        current_ratio
    }

    pub fn get_all_pools(
        &self,
        pool_ids: Option<Vec<u64>>,
        filter_type: i32,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryEarnPoolResponse> {
        let pools_response = self.get_all_pools_apr(pool_ids.clone(), filter_type, pagination)?;
        let aprs_response =
            self.get_masterchef_pool_apr(pool_ids.or(Some([].to_vec())).unwrap())?;

        match (pools_response.pools, aprs_response.data) {
            (Some(pools), aprs) => {
                // Create a map from pool_id to APR for efficient lookup
                let aprs_map: HashMap<String, PoolApr> = aprs
                    .into_iter()
                    .map(|apr| (apr.pool_id.to_string(), apr))
                    .collect();

                // Update the APR field for each pool, pool share price, add asset usd value
                // and current Pool ratio
                let pools_with_usd_values = pools
                    .into_iter()
                    .map(|mut pool: PoolResp| {
                        pool.apr = Some(
                            aprs_map
                                .get(&pool.pool_id.to_string())
                                .expect("APR is not present for given pool id")
                                .clone(),
                        );

                        pool.assets = pool
                            .assets
                            .into_iter()
                            .map(|asset| {
                                let price = self
                                    .get_asset_price(asset.token.denom.clone())
                                    .unwrap_or(Decimal::zero());
                                let usd_value = Decimal::from_atomics(asset.token.amount, 6)
                                    .map_or(Decimal::zero(), |res| res * price);

                                PoolAsset {
                                    token: asset.token.clone(),
                                    weight: asset.weight,
                                    usd_value: Some(usd_value),
                                }
                            })
                            .collect::<Vec<_>>();

                        pool.current_pool_ratio = Some(self.get_current_pool_ratio(&pool));

                        pool.share_usd_price = Some(
                            pool.tvl
                                .checked_div(
                                    Decimal::from_atomics(pool.total_shares.amount, 18)
                                        .unwrap_or(Decimal::zero()),
                                )
                                .unwrap_or_default(),
                        );

                        let usdc_entry = self.get_asset_profile("uusdc".to_string());

                        // Add USD value to every reward coin returned from chain
                        if let Ok(entry) = &usdc_entry {
                            pool.fiat_rewards = Some(
                                pool.reward_coins
                                    .iter()
                                    .map(|coin| CoinValue::from_coin(&coin, self).unwrap())
                                    .collect(),
                            );
                            if let Some(index) = pool
                                .assets
                                .iter()
                                .position(|asset| asset.token.denom == entry.entry.denom)
                            {
                                let usdc_asset = pool.assets.remove(index);
                                pool.assets.push(usdc_asset);

                                pool.current_pool_ratio_string = {
                                    let mut ratio_string = String::new();
                                    if let Some(current_pool_ratio) = &pool.current_pool_ratio {
                                        for (index, asset) in pool.assets.iter().enumerate() {
                                            if let Some(ratio) =
                                                current_pool_ratio.get(&asset.token.denom)
                                            {
                                                ratio_string.push_str(&ratio.to_string());
                                                if index < pool.assets.len() - 1 {
                                                    ratio_string.push(':');
                                                }
                                            }
                                        }
                                    }
                                    Some(ratio_string)
                                };
                            }
                        }

                        pool
                    })
                    .collect::<Vec<PoolResp>>();

                Ok(QueryEarnPoolResponse {
                    pools: Some(pools_with_usd_values),
                })
            }
            (None, _) => {
                // Return default response if either pools or APR data is missing
                Ok(QueryEarnPoolResponse {
                    pools: Some(Vec::new()), // or None, depending on how you define default
                })
            }
        }
    }

    pub fn get_asset_price(&self, asset: impl Into<String>) -> StdResult<Decimal> {
        let mut asset: String = asset.into();

        if asset == "ueden" {
            asset = "uelys".to_string()
        }

        let QueryGetEntryResponse {
            entry:
                Entry {
                    denom: usdc_denom,
                    display_name,
                    ..
                },
        } = self.get_asset_profile("uusdc".to_string())?;

        let QueryGetPriceResponse {
            price: Price {
                price: usdc_usd_price,
                ..
            },
        } = self.get_oracle_price(display_name, "".to_string(), 0)?;

        if asset == usdc_denom {
            return Ok(usdc_usd_price);
        }

        let band_ticker = match self.asset_info(asset.clone()) {
            Ok(asset_info) => Some(asset_info.asset_info.band_ticker),
            Err(_) => None,
        };

        let oracle_price = if let Some(band_ticker) = band_ticker {
            if let Ok(oracle_price) = self.get_oracle_price(band_ticker, "".to_string(), 0) {
                Some(oracle_price.price.price)
            } else {
                None
            }
        } else {
            None
        };

        // FIXME: convert first 1USDC to DENOM IN and use the result as input amount to convert DENOM IN to DENOM OUT

        //discount is set to ONE because we need to keep at 100% so it does not apply the swap fee in the price calculation
        let asset_usdc_price = match oracle_price {
            Some(price) => price,
            None => self
                .get_amm_price_by_denom(coin(1000000, asset), Decimal::one())
                .map_err(|e| {
                    StdError::generic_err(format!("get_asset_price: spot price not found:{:?}", e))
                })?,
        };

        //ATOM/USDC * USDC/USD_rate = ATOM/USD
        let price = asset_usdc_price.checked_mul(usdc_usd_price)?;

        Ok(price)
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

    pub fn masterchef_params(&self) -> StdResult<MasterchefParamsResponse> {
        let request = QueryRequest::Custom(ElysQuery::masterchef_params());
        self.querier.query(&request)
    }

    pub fn masterchef_pool_info(&self, pool_id: u64) -> StdResult<MasterchefPoolInfoResponse> {
        let request = QueryRequest::Custom(ElysQuery::masterchef_pool_info(pool_id));
        self.querier.query(&request)
    }

    /// This function retrieves pending rewards for a user from the Masterchef contract.
    ///
    /// Arguments:
    ///
    /// * `address`: The `address` parameter in the `get_masterchef_pending_rewards` function is a
    /// String type that represents the address for which you want to retrieve pending rewards from the
    /// Masterchef contract.
    ///
    /// Returns:
    ///
    /// The function `get_masterchef_pending_rewards` returns a `StdResult` containing a
    /// `MasterchefUserPendingRewardResponse`.
    pub fn get_masterchef_pending_rewards(
        &self,
        address: String,
    ) -> StdResult<MasterchefUserPendingRewardResponse> {
        self.querier.query(&QueryRequest::Custom(
            ElysQuery::masterchef_pending_rewards(address),
        ))
    }

    pub fn get_masterchef_pool_apr(&self, pool_ids: Vec<u64>) -> StdResult<QueryPoolAprsResponse> {
        let query = ElysQuery::get_masterchef_pool_apr(pool_ids);
        let request = QueryRequest::Custom(query);
        let mut response: QueryPoolAprsResponse = self.querier.query(&request).unwrap_or_default();

        for pool_apr in response.data.iter_mut() {
            let multiplier = Decimal::from_str("100").unwrap();

            pool_apr.eden_apr = pool_apr
                .eden_apr
                .checked_mul(multiplier)
                .unwrap_or_default();
            pool_apr.usdc_apr = pool_apr
                .usdc_apr
                .checked_mul(multiplier)
                .unwrap_or_default();
            pool_apr.total_apr = pool_apr
                .total_apr
                .checked_mul(multiplier)
                .unwrap_or_default();
        }

        Ok(response)
    }

    pub fn get_estaking_rewards(&self, address: String) -> StdResult<EstakingRewardsResponse> {
        let query = ElysQuery::EstakingRewards {
            address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(query);
        self.querier.query(&request)
    }

    pub fn get_masterchef_stable_stake_apr(
        &self,
        denom: String,
    ) -> StdResult<QueryStableStakeAprResponse> {
        self.querier.query(&QueryRequest::Custom(
            ElysQuery::get_masterchef_stable_stake_apr(denom),
        ))
    }

    pub fn leveragelp_params(&self) -> StdResult<LeveragelpParamsResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_params());
        let raw_resp: LeveragelpParamsResponseRaw = self.querier.query(&req)?;
        let params = match raw_resp.params {
            Some(raw_params) => Some(LeveragelpParams {
                leverage_max: raw_params.leverage_max.unwrap_or(Decimal::zero()),
                max_open_positions: raw_params.max_open_positions.unwrap_or(0),
                pool_open_threshold: raw_params.pool_open_threshold.unwrap_or(Decimal::zero()),
                safety_factor: raw_params.safety_factor.unwrap_or(Decimal::zero()),
                whitelisting_enabled: raw_params.whitelisting_enabled.unwrap_or(false),
                epoch_length: raw_params.epoch_length.unwrap_or(0),
            }),
            None => None,
        };
        Ok(LeveragelpParamsResponse { params })
    }
    pub fn leveragelp_query_positions(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<LeveragelpPositionsResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_query_positions(pagination));
        let raw_resp: LeveragelpPositionsResponseRaw = self.querier.query(&req)?;
        let positions = raw_resp.positions.unwrap_or(vec![]);
        Ok(LeveragelpPositionsResponse {
            positions,
            pagination: raw_resp.pagination,
        })
    }
    pub fn leveragelp_query_positions_by_pool(
        &self,
        amm_pool_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<LeveragelpPositionsResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_query_positions_by_pool(
            amm_pool_id,
            pagination,
        ));
        let raw_resp: LeveragelpPositionsResponseRaw = self.querier.query(&req)?;
        let positions = raw_resp.positions.unwrap_or(vec![]);
        Ok(LeveragelpPositionsResponse {
            positions,
            pagination: raw_resp.pagination,
        })
    }
    pub fn leveragelp_get_status(&self) -> StdResult<LeveragelpStatusResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_get_status());
        let raw_resp: LeveragelpStatusResponseRaw = self.querier.query(&req)?;
        let resp = LeveragelpStatusResponse {
            open_position_count: raw_resp.open_position_count.unwrap_or(0),
            lifetime_position_count: raw_resp.lifetime_position_count.unwrap_or(0),
        };
        Ok(resp)
    }
    pub fn leveragelp_query_positions_for_address(
        &self,
        address: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> StdResult<LeveragelpPositionsResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_query_positions_for_address(
            address.into(),
            pagination,
        ));
        let raw_resp: LeveragelpPositionsResponseRaw = self.querier.query(&req)?;
        let positions = raw_resp.positions.unwrap_or(vec![]);
        Ok(LeveragelpPositionsResponse {
            positions,
            pagination: raw_resp.pagination,
        })
    }
    pub fn leveragelp_get_whitelist(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<LeveragelpWhitelistResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_get_whitelist(pagination));
        let raw_resp: LeveragelpWhitelistResponseRaw = self.querier.query(&req)?;
        let resp = LeveragelpWhitelistResponse {
            whitelist: raw_resp.whitelist.unwrap_or(vec![]),
            pagination: raw_resp.pagination,
        };
        Ok(resp)
    }
    pub fn leveragelp_is_whitelisted(
        &self,
        address: impl Into<String>,
    ) -> StdResult<LeveragelpIsWhitelistedResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_is_whitelisted(address.into()));
        let raw_resp: LeveragelpIsWhitelistedResponseRaw = self.querier.query(&req)?;
        let resp = LeveragelpIsWhitelistedResponse {
            address: raw_resp.address,
            is_whitelisted: raw_resp.is_whitelisted.unwrap_or(false),
        };
        Ok(resp)
    }
    pub fn leveragelp_pool(&self, index: u64) -> StdResult<LeveragelpPoolResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_pool(index));
        let raw_resp: LeveragelpPoolResponseRaw = self.querier.query(&req)?;
        let resp = LeveragelpPoolResponse {
            pool: LeveragelpPool {
                amm_pool_id: raw_resp.pool.amm_pool_id,
                health: raw_resp.pool.health,
                enabled: raw_resp.pool.enabled.unwrap_or(false),
                closed: raw_resp.pool.closed.unwrap_or(false),
                leveraged_lp_amount: raw_resp.pool.leveraged_lp_amount,
                leverage_max: raw_resp.pool.leverage_max,
            },
        };
        Ok(resp)
    }
    pub fn leveragelp_pools(
        &self,
        pagination: Option<PageRequest>,
    ) -> StdResult<LeveragelpPoolsResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_pools(pagination));
        let raw_resp: LeveragelpPoolsResponseRaw = self.querier.query(&req)?;
        let pool: Vec<LeveragelpPool> = raw_resp
            .pool
            .into_iter()
            .map(|pool| LeveragelpPool {
                amm_pool_id: pool.amm_pool_id,
                health: pool.health,
                enabled: pool.enabled.unwrap_or(false),
                closed: pool.closed.unwrap_or(false),
                leveraged_lp_amount: pool.leveraged_lp_amount,
                leverage_max: pool.leverage_max,
            })
            .collect();
        let resp = LeveragelpPoolsResponse {
            pool,
            pagination: raw_resp.pagination,
        };
        Ok(resp)
    }
    pub fn leveragelp_position(
        &self,
        address: impl Into<String>,
        id: u64,
    ) -> StdResult<LeveragelpPositionResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_position(address.into(), id));
        self.querier.query(&req)
    }
    pub fn commitment_number_of_commitments(
        &self,
    ) -> StdResult<CommitmentNumberOfCommitmentsResponse> {
        let req = QueryRequest::Custom(ElysQuery::commitment_number_of_commitments());
        let CommitmentNumberOfCommitmentsResponseRaw { number } = self.querier.query(&req)?;
        Ok(CommitmentNumberOfCommitmentsResponse {
            number: number.unwrap_or(0),
        })
    }

    pub fn leveragelp_open_est(
        &self,
        collateral_asset: String,
        collateral_amount: Int128,
        amm_pool_id: u64,
        leverage: Decimal,
    ) -> StdResult<LeveragelpOpenEstimationResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_open_est(
            collateral_asset,
            collateral_amount,
            amm_pool_id,
            leverage,
        ));
        self.querier.query(&req)
    }
    pub fn leveragelp_close_est(
        &self,
        owner: String,
        id: u64,
        lp_amount: Int128,
    ) -> StdResult<LeveragelpCloseEstimationResponse> {
        let req = QueryRequest::Custom(ElysQuery::leveragelp_close_est(owner, id, lp_amount));
        self.querier.query(&req)
    }
    pub fn get_all_pools_apr(
        &self,
        pool_ids: Option<Vec<u64>>,
        filter_type: i32,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryEarnPoolResponse> {
        let pools_query = ElysQuery::get_all_pools(pool_ids, filter_type, pagination);
        let pools_request: QueryRequest<ElysQuery> = QueryRequest::Custom(pools_query);

        self.querier.query(&pools_request)
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
