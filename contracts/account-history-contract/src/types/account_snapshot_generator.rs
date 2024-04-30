use std::collections::HashMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, Coin, DecCoin, Decimal, Decimal256, Deps, Env, QuerierWrapper, StdError, StdResult,
    Uint128,
};
use cw_utils::Expiration;
use elys_bindings::{
    account_history::{
        msg::query_resp::{GetRewardsResp, StakeAssetBalanceBreakdown, StakedAssetsResponse},
        types::{
            earn_program::{EdenEarnProgram, ElysEarnProgram, UsdcEarnProgram},
            AccountSnapshot, BalanceReward, CoinValue, ElysDenom, LiquidAsset, Metadata,
            PerpetualAsset, PerpetualAssets, PoolBalances, Portfolio, PortfolioBalanceSnapshot,
            Reward, StakedAssets, TotalBalance,
        },
    },
    query_resp::{CommittedTokens, PoolFilterType, PoolResp, QueryUserPoolResponse, UserPoolResp},
    trade_shield::{
        msg::{
            query_resp::{
                GetPerpetualOrdersResp, GetPerpetualPositionsForAddressResp, GetSpotOrdersResp,
            },
            QueryMsg::{GetPerpetualOrders, GetSpotOrders, PerpetualGetPositionsForAddress},
        },
        types::{PerpetualOrder, PerpetualOrderPlus, PerpetualOrderType, SpotOrder, Status},
    },
    types::{BalanceAvailable, EarnType},
    ElysQuerier, ElysQuery,
};

use crate::{
    action::query::{
        get_eden_boost_earn_program_details, get_eden_earn_program_details,
        get_elys_earn_program_details, get_pools, get_usdc_earn_program_details,
    },
    states::{EXPIRATION, METADATA, TRADE_SHIELD_ADDRESS},
};

#[cw_serde]
pub struct AccountSnapshotGenerator {
    pub trade_shield_address: Option<String>,
    pub expiration: Expiration,
    pub metadata: Metadata,
}

impl AccountSnapshotGenerator {
    pub fn new(deps: &Deps<ElysQuery>) -> StdResult<Self> {
        let expiration = EXPIRATION.load(deps.storage)?;
        let trade_shield_address = TRADE_SHIELD_ADDRESS.load(deps.storage)?;
        let metadata = METADATA.load(deps.storage)?;

        Ok(Self {
            trade_shield_address,
            expiration,
            metadata,
        })
    }

    pub fn generate_portfolio_balance_snapshot_for_address(
        &self,
        querier: &ElysQuerier,
        deps: &Deps<ElysQuery>,
        env: &Env,
        address: &String,
    ) -> StdResult<PortfolioBalanceSnapshot> {
        let snapshot = self.generate_account_snapshot_for_address(querier, deps, env, address)?;

        Ok(PortfolioBalanceSnapshot {
            date: snapshot.date,
            portfolio_balance_usd: snapshot.portfolio.balance_usd.clone(),
            total_balance_usd: snapshot.total_balance.total_balance.clone(),
        })
    }

    pub fn generate_account_snapshot_for_address(
        &self,
        querier: &ElysQuerier,
        deps: &Deps<ElysQuery>,
        env: &Env,
        address: &String,
    ) -> StdResult<AccountSnapshot> {
        let liquid_assets_response = self.get_liquid_assets(&deps, querier, &address)?;
        let staked_assets_response = self.get_staked_assets(&deps, &address)?;
        let rewards_response = self.get_rewards(&deps, &address)?;
        let perpetual_response = self.get_perpetuals(&deps, &address)?;
        let pool_balances_response = self.get_pool_balances(&deps, &address)?;

        let date = match self.expiration {
            Expiration::AtHeight(_) => Expiration::AtHeight(env.block.height),
            Expiration::AtTime(_) => Expiration::AtTime(env.block.time),
            Expiration::Never {} => panic!("never expire"),
        };

        let mut total_liquidity_position_balance = Decimal256::zero();
        for pool in pool_balances_response.pools.iter() {
            total_liquidity_position_balance =
                total_liquidity_position_balance.checked_add(Decimal256::from(pool.available))?;
        }

        let reward = rewards_response.rewards_map;
        let portfolio_usd = liquid_assets_response
            .total_liquid_asset_balance
            .amount
            .checked_add(Decimal256::from(staked_assets_response.total_balance))?
            .checked_add(
                perpetual_response
                    .total_perpetual_asset_balance
                    .amount
                    .clone(),
            )?
            .checked_add(total_liquidity_position_balance)?;

        let reward_usd = Decimal256::from(reward.total_usd.clone());
        let total_balance = portfolio_usd.checked_add(reward_usd.clone())?;

        // Adds the records all the time as we should return data to the FE even if it is 0 balanced.
        Ok(AccountSnapshot {
            date,
            total_balance: TotalBalance {
                total_balance,
                portfolio_usd: portfolio_usd.clone(),
                reward_usd,
            },
            portfolio: Portfolio {
                balance_usd: portfolio_usd,
                liquid_assets_usd: liquid_assets_response
                    .total_liquid_asset_balance
                    .amount
                    .clone(),
                staked_committed_usd: Decimal256::from(
                    staked_assets_response.total_balance,
                ),
                liquidity_positions_usd: total_liquidity_position_balance,
                leverage_lp_usd: Decimal256::zero(),
                perpetual_assets_usd: perpetual_response
                    .total_perpetual_asset_balance
                    .amount
                    .clone(),
                usdc_earn_usd: Decimal256::zero(),
                borrows_usd: Decimal256::zero(),
                stake_balance_breakdown: staked_assets_response.balance_break_down,
            },
            reward,
            pool_balances: PoolBalances {
                balances: pool_balances_response.pools,
            },
            liquid_asset: liquid_assets_response,
            staked_assets: staked_assets_response.staked_assets,
            perpetual_assets: perpetual_response,
        })
    }

    pub fn get_pools_user_rewards(
        &self,
        deps: &Deps<ElysQuery>,
        address: &String,
    ) -> (Decimal, HashMap<String, BalanceAvailable>) {
        let querier = ElysQuerier::new(&deps.querier);

        let usdc_rewards = querier.get_sub_bucket_rewards_balance(
            address.clone(),
            self.metadata.usdc_denom.to_owned(),
            EarnType::LiquidityMiningProgram as i32,
        );
        let eden_rewards = querier.get_sub_bucket_rewards_balance(
            address.clone(),
            ElysDenom::Eden.as_str().to_string(),
            EarnType::LiquidityMiningProgram as i32,
        );

        let mut total = Decimal::zero();
        let mut breakdown: HashMap<String, BalanceAvailable> = HashMap::new();
        match usdc_rewards {
            Ok(reward) => {
                let decimal_reward = Decimal::from_atomics(reward.amount, 6).unwrap();
                // usd_value is not being converted on chain
                let fiat_reward = decimal_reward * self.metadata.uusdc_usd_price;

                total = total
                    .checked_add(fiat_reward)
                    .map_or(Decimal::zero(), |res| res);
                breakdown.insert(
                    self.metadata.usdc_denom.to_owned(),
                    BalanceAvailable {
                        amount: reward.amount,
                        usd_amount: fiat_reward,
                    },
                );
            }
            Err(_) => {}
        };

        match eden_rewards {
            Ok(reward) => {
                let decimal_reward = Decimal::from_atomics(reward.amount, 6).unwrap();
                // usd_value is not being converted on chain.
                // uelys_price_in_uusdc incorrectly named, it should be usd...
                let fiat_reward = decimal_reward * self.metadata.uelys_price_in_uusdc;

                total = total
                    .checked_add(fiat_reward)
                    .map_or(Decimal::zero(), |res| res);
                breakdown.insert(
                    ElysDenom::Eden.as_str().to_string(),
                    BalanceAvailable {
                        amount: reward.amount,
                        usd_amount: fiat_reward,
                    },
                );
            }
            Err(_) => {}
        };

        (total, breakdown)
    }

    pub fn get_pool_balances(
        &self,
        deps: &Deps<ElysQuery>,
        address: &String,
    ) -> StdResult<QueryUserPoolResponse> {
        let querier = ElysQuerier::new(&deps.querier);
        let commitments = querier.get_commitments(address.clone())?.commitments;

        struct IdSortedPoolBalance {
            pub id: u64,
            pub balance: CommittedTokens,
        }

        let pool_balances: Vec<CommittedTokens> = match commitments.committed_tokens {
            Some(res) => res
                .iter()
                .filter(|coin| coin.denom.starts_with("amm/pool/"))
                .cloned()
                .collect(),
            None => vec![],
        };

        let pool_data: Vec<IdSortedPoolBalance> = pool_balances
            .iter()
            .map(|coin| {
                let id = coin
                    .denom
                    .split("/")
                    .last()
                    .unwrap_or("0")
                    .parse::<u64>()
                    .unwrap_or(0u64);
                IdSortedPoolBalance {
                    id,
                    balance: coin.clone(),
                }
            })
            .collect();

        // For each pool_data, fetch the pool with that ID
        let mut pool_resp: Vec<UserPoolResp> = Vec::new();
        for user_pool in pool_data {
            let pool_id = user_pool.id;
            let pool = get_pools(*deps, Some(vec![pool_id]), PoolFilterType::FilterAll, None)?;
            let pool = pool
                .pools
                .unwrap_or_default()
                .first()
                .map_or(PoolResp::default(), |pool| pool.clone());

            let balance_uint = Uint128::new(user_pool.balance.amount.i128() as u128);
            let share_price = pool.share_usd_price.or(Some(Decimal::zero())).unwrap();

            // Assumes that pool.assets are in the desired displaying sort order.
            let balance_breakdown = pool
                .assets
                .clone()
                .into_iter()
                .map(|asset| match pool.current_pool_ratio.clone() {
                    Some(ratios) => {
                        let denom = asset.token.denom.clone();
                        let ratio = ratios.get(&denom);
                        let asset_price = querier.get_asset_price(denom.clone());

                        match (asset_price, ratio) {
                            (Ok(price), Some(ratio)) => {
                                let asset_shares =
                                    Decimal::from_atomics(balance_uint, 18).unwrap() * ratio;
                                let shares_usd = asset_shares * share_price;
                                let asset_amount = shares_usd / price;

                                Some(CoinValue::new(denom, asset_amount, price, shares_usd))
                            }
                            (_, _) => None,
                        }
                    }
                    _ => None,
                })
                .collect();

            pool_resp.push(UserPoolResp {
                pool,
                balance: user_pool.balance,
                available: Decimal::from_atomics(balance_uint, 18).unwrap() * share_price,
                balance_breakdown,
            });
        }

        let (rewards, rewards_breakdown) = self.get_pools_user_rewards(&deps, address);

        Ok(QueryUserPoolResponse {
            pools: pool_resp,
            rewards,
            rewards_breakdown,
        })
    }

    pub fn get_liquid_assets(
        &self,
        deps: &Deps<ElysQuery>,
        querier: &ElysQuerier,
        address: &String,
    ) -> StdResult<LiquidAsset> {
        let mut account_balances = deps.querier.query_all_balances(address)?;
        let orders_balances =
            self.get_all_orders(&deps.querier, &self.trade_shield_address, &address)?;

        let eden_program = get_eden_earn_program_details(
            deps,
            Some(address.to_owned()),
            ElysDenom::Eden.as_str().to_string(),
            self.metadata.usdc_denom.to_owned(),
            self.metadata.uusdc_usd_price,
            self.metadata.uelys_price_in_uusdc,
            self.metadata.usdc_apr_eden.to_owned(),
            self.metadata.eden_apr_eden.to_owned(),
            self.metadata.edenb_apr_eden.to_owned(),
        )
        .unwrap_or_default();

        let available = eden_program.data.available.unwrap_or_default();
        let eden_coin = Coin::new(u128::from(available.amount), ElysDenom::Eden.as_str());
        if available.amount > Uint128::zero() {
            account_balances.push(eden_coin);
        }

        let available_asset_balance: Vec<CoinValue> = account_balances
            .iter()
            .filter_map(|coin| {
                match CoinValue::from_coin(coin, querier, &self.metadata.usdc_denom) {
                    Ok(res) => Some(res),
                    Err(_) => None,
                }
            })
            .collect();

        let in_orders_asset_balance: Vec<CoinValue> = orders_balances
            .iter()
            .filter_map(|coin| {
                match CoinValue::from_coin(coin, querier, &self.metadata.usdc_denom) {
                    Ok(res) => Some(res),
                    Err(_) => None,
                }
            })
            .collect();

        let mut total_available_balance =
            DecCoin::new(Decimal256::zero(), &self.metadata.usdc_denom);
        let mut total_in_orders_balance =
            DecCoin::new(Decimal256::zero(), &self.metadata.usdc_denom);

        for balance in &available_asset_balance {
            total_available_balance.amount = total_available_balance
                .amount
                .checked_add(Decimal256::from(balance.amount_usd.clone()))?
        }

        for balance in &in_orders_asset_balance {
            total_in_orders_balance.amount = total_in_orders_balance
                .amount
                .checked_add(Decimal256::from(balance.amount_usd.clone()))?
        }

        let mut total_value_per_asset: HashMap<&String, CoinValue> = HashMap::new();

        for available in available_asset_balance.iter() {
            total_value_per_asset
                .entry(&available.denom)
                .and_modify(|e| {
                    e.amount_token += available.amount_token.clone();
                    e.amount_usd += available.amount_usd.clone();
                })
                .or_insert_with(|| available.clone());
        }

        for in_order in in_orders_asset_balance.iter() {
            total_value_per_asset
                .entry(&in_order.denom)
                .and_modify(|e| {
                    e.amount_token += in_order.amount_token.clone();
                    e.amount_usd += in_order.amount_usd.clone();
                })
                .or_insert_with(|| in_order.clone());
        }

        let total_value_per_asset: Vec<CoinValue> =
            total_value_per_asset.values().cloned().collect();

        let total_liquid_asset_balance = DecCoin::new(
            Decimal256::from(
                total_value_per_asset
                    .iter()
                    .map(|v| v.amount_usd)
                    .fold(Decimal::zero(), |acc, item| acc + item),
            ),
            &self.metadata.usdc_denom,
        );

        Ok(LiquidAsset {
            total_liquid_asset_balance,
            total_available_balance,
            total_in_orders_balance,
            available_asset_balance,
            in_orders_asset_balance,
            total_value_per_asset,
        })
    }

    pub fn get_staked_assets(
        &self,
        deps: &Deps<ElysQuery>,
        address: &String,
    ) -> StdResult<StakedAssetsResponse> {
        // create staked_assets variable that is a StakedAssets struct
        let mut staked_assets = StakedAssets::default();
        let mut total_staked_balance = Decimal::zero();

        let usdc_details = get_usdc_earn_program_details(
            deps,
            Some(address.to_owned()),
            self.metadata.usdc_denom.to_owned(),
            self.metadata.usdc_base_denom.to_owned(),
            self.metadata.uusdc_usd_price
        )
        .unwrap_or_default();

        // usdc program
        let staked_asset_usdc = usdc_details.data.clone();
        total_staked_balance = total_staked_balance
            .checked_add(match staked_asset_usdc.clone() {
                UsdcEarnProgram {
                    staked: Some(r), ..
                } => r.usd_amount,
                _ => Decimal::zero(),
            })
            .unwrap_or_default();
        staked_assets.usdc_earn_program = staked_asset_usdc;

        // elys program
        let elys_details = get_elys_earn_program_details(
            deps,
            Some(address.to_owned()),
            ElysDenom::Elys.as_str().to_string(),
            self.metadata.usdc_denom.to_owned(),
            self.metadata.uusdc_usd_price,
            self.metadata.uelys_price_in_uusdc,
            self.metadata.usdc_apr_elys.to_owned(),
            self.metadata.eden_apr_elys.to_owned(),
            self.metadata.edenb_apr_elys.to_owned(),
        )
        .unwrap_or_default();

        let staked_asset_elys = elys_details.data;
        total_staked_balance = total_staked_balance
            .checked_add(match staked_asset_elys.clone() {
                ElysEarnProgram {
                    staked: Some(r), ..
                } => r.usd_amount,
                _ => Decimal::zero(),
            })
            .unwrap_or_default();
        staked_assets.elys_earn_program = staked_asset_elys.clone();
        let unstaking = if let Some(unstaked_positions) = staked_asset_elys.unstaked_positions {
            let total_usd_amount =
                unstaked_positions
                    .iter()
                    .fold(Decimal::zero(), |acc, position| {
                        // Accumulate the usd_amount from each UnstakedPosition
                        acc.checked_add(position.unstaked.usd_amount)
                            .unwrap_or_default()
                    });
            total_usd_amount
        } else {
            Decimal::zero()
        };

        // eden program
        let eden_details = get_eden_earn_program_details(
            deps,
            Some(address.to_owned()),
            ElysDenom::Eden.as_str().to_string(),
            self.metadata.usdc_denom.to_owned(),
            self.metadata.uusdc_usd_price,
            self.metadata.uelys_price_in_uusdc,
            self.metadata.usdc_apr_eden.to_owned(),
            self.metadata.eden_apr_eden.to_owned(),
            self.metadata.edenb_apr_eden.to_owned(),
        )
        .unwrap_or_default();

        let staked_asset_eden = eden_details.data;
        total_staked_balance = total_staked_balance
            .checked_add(match staked_asset_eden.clone() {
                EdenEarnProgram {
                    staked: Some(r), ..
                } => r.usd_amount,
                _ => Decimal::zero(),
            })
            .unwrap_or_default();
        let vesting = staked_asset_eden.vesting.usd_amount;

        staked_assets.eden_earn_program = staked_asset_eden;

        let edenb_details = get_eden_boost_earn_program_details(
            deps,
            Some(address.to_owned()),
            ElysDenom::EdenBoost.as_str().to_string(),
            self.metadata.usdc_denom.to_owned(),
            self.metadata.usdc_apr_edenb.to_owned(),
            self.metadata.eden_apr_edenb.to_owned(),
        )
        .unwrap_or_default();

        let staked_asset_edenb = edenb_details.data;
        staked_assets.eden_boost_earn_program = staked_asset_edenb;
        let balance_break_down = StakeAssetBalanceBreakdown {
            staked: Decimal::from(total_staked_balance),
            unstaking,
            vesting,
        };

        Ok(StakedAssetsResponse {
            staked_assets,
            total_staked_balance: DecCoin::new(
                Decimal256::from(total_staked_balance),
                self.metadata.usdc_denom.to_owned(),
            ),
            total_balance: balance_break_down.total(),
            balance_break_down,
        })
    }

    pub fn get_all_orders(
        &self,
        querier: &QuerierWrapper<ElysQuery>,
        trade_shield_address: &Option<String>,
        owner: &String,
    ) -> StdResult<Vec<Coin>> {
        let trade_shield_address = match trade_shield_address {
            Some(trade_shield_address) => trade_shield_address,
            None => return Ok(vec![]),
        };

        let spot_order: GetSpotOrdersResp = querier
            .query_wasm_smart(
                trade_shield_address,
                &GetSpotOrders {
                    pagination: None,
                    order_owner: Some(owner.clone()),
                    order_type: None,
                    order_status: Some(Status::Pending),
                },
            )
            .map_err(|e| StdError::generic_err(format!("GetSpotOrders failed {}", e)))?;
        let perpetual_order: GetPerpetualOrdersResp = querier
            .query_wasm_smart(
                trade_shield_address,
                &GetPerpetualOrders {
                    pagination: None,
                    order_owner: Some(owner.clone()),
                    order_type: Some(PerpetualOrderType::LimitOpen),
                    order_status: Some(Status::Pending),
                },
            )
            .map_err(|e| StdError::generic_err(format!("GetPerpetualOrders failed {}", e)))?;
        let mut map: HashMap<String, Uint128> = HashMap::new();

        for SpotOrder { order_amount, .. } in spot_order.orders {
            map.entry(order_amount.denom)
                .and_modify(|e| *e += order_amount.amount)
                .or_insert(order_amount.amount);
        }
        for PerpetualOrderPlus {
            order: PerpetualOrder { collateral, .. },
            ..
        } in perpetual_order.orders
        {
            map.entry(collateral.denom)
                .and_modify(|e| *e += collateral.amount)
                .or_insert(collateral.amount);
        }

        let consolidated_coins: Vec<Coin> = map
            .into_iter()
            .map(|(denom, amount)| Coin { denom, amount })
            .collect();
        Ok(consolidated_coins)
    }

    pub fn get_perpetuals(
        &self,
        deps: &Deps<ElysQuery>,
        address: &String,
    ) -> StdResult<PerpetualAssets> {
        let trade_shield_address = match self.trade_shield_address.clone() {
            Some(trade_shield_address) => trade_shield_address,
            None => return Ok(PerpetualAssets::default()),
        };

        let GetPerpetualPositionsForAddressResp { mtps, .. } = deps
            .querier
            .query_wasm_smart(
                trade_shield_address,
                &PerpetualGetPositionsForAddress {
                    address: address.to_string(),
                    pagination: None,
                },
            )
            .map_err(|_| StdError::generic_err("an error occurred while getting perpetuals"))?;
        let mut perpetual_vec: Vec<PerpetualAsset> = vec![];
        let querier = ElysQuerier::new(&deps.querier);

        for mtp in mtps {
            match PerpetualAsset::new(mtp, self.metadata.usdc_denom.to_owned(), &querier) {
                Ok(perpetual_asset) => perpetual_vec.push(perpetual_asset),
                Err(_) => continue,
            }
        }

        let total_perpetual_asset_balance_amount = perpetual_vec
            .iter()
            .map(|perpetual| perpetual.size.amount)
            .fold(Decimal256::zero(), |acc, item| acc + item);
        let total_perpetual_asset_balance = DecCoin::new(
            total_perpetual_asset_balance_amount,
            self.metadata.usdc_denom.to_owned(),
        );

        Ok(PerpetualAssets {
            total_perpetual_asset_balance,
            perpetual_asset: perpetual_vec,
        })
    }

    pub fn get_rewards(
        &self,
        deps: &Deps<ElysQuery>,
        address: &String,
    ) -> StdResult<GetRewardsResp> {
        let querier = ElysQuerier::new(&deps.querier);
        let commitments = querier.get_commitments(address.to_string())?;

        let denom_usdc_entry = querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
        let denom_uusdc = denom_usdc_entry.entry.denom;
        let usdc_display_denom = denom_usdc_entry.entry.display_name;

        let denom_uelys = ElysDenom::Elys.as_str().to_string();
        let denom_ueden = ElysDenom::Eden.as_str().to_string();
        let denom_uedenb = ElysDenom::EdenBoost.as_str().to_string();

        let usdc_oracle_price = querier.get_oracle_price(
            usdc_display_denom.clone(),
            ElysDenom::AnySource.as_str().to_string(),
            0,
        )?;
        let usdc_price = usdc_oracle_price
            .price
            .price
            .checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap())
            .unwrap_or_default();

        let mut balance_rewards: Vec<BalanceReward> = vec![];
        let mut rewards = Reward {
            usdc_usd: Decimal::zero(),
            eden_usd: Decimal::zero(),
            eden_boost: Uint128::zero(),
            other_usd: Decimal::zero(),
            total_usd: Decimal::zero(),
        };

        match commitments.commitments.rewards_unclaimed {
            Some(rewards_unclaimed) => {
                for reward in rewards_unclaimed {
                    // uusdc
                    if reward.denom == denom_uusdc {
                        let usdc_rewards =
                            Decimal::from_atomics(reward.amount, 0).unwrap_or_default();
                        let rewards_in_usd = usdc_rewards.checked_mul(usdc_price)?;

                        balance_rewards.push(BalanceReward {
                            asset: denom_usdc_entry.entry.base_denom.clone(),
                            amount: reward.amount,
                            usd_amount: Some(rewards_in_usd),
                        });

                        rewards.usdc_usd = rewards_in_usd;
                        rewards.total_usd = rewards
                            .total_usd
                            .checked_add(rewards.usdc_usd)
                            .unwrap_or_default();

                        continue;
                    }

                    // ueden
                    if reward.denom == denom_ueden {
                        // if it is eden, we should elys denom instead of ueden as it is not available in LP pool and has the same value with elys.
                        let reward_in_elys = coin(reward.amount.u128(), denom_uelys.to_owned());
                        let price = querier.get_asset_price(reward_in_elys.denom)?;

                        let amount = coin(
                            (price
                                .checked_mul(
                                    Decimal::from_atomics(reward_in_elys.amount, 0).map_err(
                                        |_| {
                                            StdError::generic_err(format!(
                                                "failed to convert to decimal"
                                            ))
                                        },
                                    )?,
                                )
                                .map_err(|e| {
                                    StdError::generic_err(format!(
                                        "failed to get_asset_price: {}",
                                        e
                                    ))
                                })?)
                            .to_uint_floor()
                            .u128(),
                            &denom_uusdc,
                        );
                        let rewards_in_usdc =
                            Decimal::from_atomics(amount.amount, 0).unwrap_or_default();
                        let rewards_in_usd =
                            rewards_in_usdc.checked_mul(usdc_price).unwrap_or_default();

                        balance_rewards.push(BalanceReward {
                            asset: denom_ueden.clone(),
                            amount: reward.amount,
                            usd_amount: Some(rewards_in_usd),
                        });

                        rewards.eden_usd = rewards_in_usd;
                        rewards.total_usd = rewards
                            .total_usd
                            .checked_add(rewards.eden_usd)
                            .unwrap_or_default();
                        continue;
                    }

                    // uedenb - we don't value eden boost in usd.
                    if reward.denom == denom_uedenb {
                        balance_rewards.push(BalanceReward {
                            asset: denom_uedenb.clone(),
                            amount: reward.amount,
                            usd_amount: None,
                        });
                        rewards.eden_boost = reward.amount;
                        continue;
                    }

                    // We accumulate other denoms in a single usd.
                    let price = querier.get_asset_price(reward.denom)?;

                    let amount = coin(
                        (price
                            .checked_mul(Decimal::from_atomics(reward.amount, 0).map_err(|_| {
                                StdError::generic_err(format!("failed to convert to decimal"))
                            })?)
                            .map_err(|e| {
                                StdError::generic_err(format!("failed to get_asset_price: {}", e))
                            })?)
                        .to_uint_floor()
                        .u128(),
                        &denom_uusdc,
                    );
                    let rewards_in_usdc =
                        Decimal::from_atomics(amount.amount, 0).unwrap_or_default();
                    let rewards_in_usd =
                        rewards_in_usdc.checked_mul(usdc_price).unwrap_or_default();

                    rewards.other_usd = rewards
                        .other_usd
                        .checked_add(rewards_in_usd)
                        .unwrap_or_default();
                    rewards.total_usd = rewards
                        .total_usd
                        .checked_add(rewards_in_usd)
                        .unwrap_or_default();

                    balance_rewards.push(BalanceReward {
                        asset: amount.denom,
                        amount: reward.amount,
                        usd_amount: Some(rewards_in_usd),
                    });
                }
            }
            None => {
                return Ok(GetRewardsResp {
                    rewards_map: AccountSnapshot::zero(&denom_uusdc).reward,
                    rewards: balance_rewards,
                });
            }
        }

        let resp = GetRewardsResp {
            rewards_map: rewards,
            rewards: balance_rewards,
        };
        Ok(resp)
    }
}
