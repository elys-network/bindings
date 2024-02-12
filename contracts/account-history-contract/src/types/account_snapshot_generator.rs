use std::collections::HashMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, BlockInfo, Coin, DecCoin, Decimal, Decimal256, Deps, Env, QuerierWrapper, StdError,
    StdResult, Uint128,
};
use cw_utils::Expiration;
use elys_bindings::{
    account_history::{
        msg::query_resp::{GetRewardsResp, StakedAssetsResponse},
        types::{
            earn_program::{
                EdenBoostEarnProgram, EdenEarnProgram, ElysEarnProgram, UsdcEarnProgram,
            },
            AccountSnapshot, CoinValue, ElysDenom, LiquidAsset, PerpetualAsset, PerpetualAssets,
            Portfolio, Reward, StakedAssets, TotalBalance,
        },
    },
    query_resp::QueryAprResponse,
    trade_shield::{
        msg::query_resp::{
            GetPerpetualOrdersResp, GetPerpetualPositionsForAddressResp, GetSpotOrdersResp,
        },
        msg::QueryMsg::{GetPerpetualOrders, GetSpotOrders, PerpetualGetPositionsForAddress},
        types::{PerpetualOrder, PerpetualOrderType, SpotOrder, Status},
    },
    types::EarnType,
    ElysQuerier, ElysQuery,
};

use crate::action::query::{
    get_eden_boost_earn_program_details, get_eden_earn_program_details,
    get_elys_earn_program_details, get_usdc_earn_program_details,
};

#[cw_serde]
pub struct AccountSnapshotGenerator {
    pub trade_shield_address: Option<String>,
    pub expiration: Expiration,
    pub usdc_denom: String,
    pub usdc_base_denom: String,
    pub usdc_display_denom: String,
    pub usdc_decimal: u64,
    pub eden_decimal: u64,
    pub uusdc_usd_price: Decimal,
    pub uelys_price_in_uusdc: Decimal,
    pub usdc_apr_usdc: QueryAprResponse,
    pub eden_apr_usdc: QueryAprResponse,
    pub usdc_apr_edenb: QueryAprResponse,
    pub eden_apr_edenb: QueryAprResponse,
    pub usdc_apr_eden: QueryAprResponse,
    pub eden_apr_eden: QueryAprResponse,
    pub edenb_apr_eden: QueryAprResponse,
    pub usdc_apr_elys: QueryAprResponse,
    pub eden_apr_elys: QueryAprResponse,
    pub edenb_apr_elys: QueryAprResponse,
}

impl AccountSnapshotGenerator {
    pub fn new(
        querier: &ElysQuerier,
        trade_shield_address: Option<String>,
        expiration: Expiration,
    ) -> StdResult<Self> {
        let usdc_denom_entry = querier
            .get_asset_profile(ElysDenom::Usdc.as_str().to_string())
            .map_err(|_| StdError::generic_err("an error occurred while getting usdc denom"))?;
        let usdc_denom = usdc_denom_entry.entry.denom;
        let usdc_base_denom = usdc_denom_entry.entry.base_denom;
        let usdc_display_denom = usdc_denom_entry.entry.display_name;
        let usdc_decimal = u64::checked_pow(10, usdc_denom_entry.entry.decimals as u32).unwrap();

        let eden_denom_entry = querier
            .get_asset_profile(ElysDenom::Eden.as_str().to_string())
            .map_err(|_| StdError::generic_err("an error occurred while getting eden denom"))?;
        let usdc_oracle_price = querier
            .get_oracle_price(
                usdc_display_denom.clone(),
                ElysDenom::AnySource.as_str().to_string(),
                0,
            )
            .map_err(|_| StdError::generic_err("an error occurred while getting usdc price"))?;

        Ok(Self {
            trade_shield_address,
            expiration,
            usdc_denom,
            usdc_base_denom,
            usdc_display_denom,
            usdc_decimal,
            eden_decimal: u64::checked_pow(10, eden_denom_entry.entry.decimals as u32).unwrap(),
            uusdc_usd_price: usdc_oracle_price
                .price
                .price
                .checked_div(Decimal::from_atomics(Uint128::new(usdc_decimal as u128), 0).unwrap())
                .unwrap(),
            uelys_price_in_uusdc: querier.get_asset_price(ElysDenom::Elys.as_str())?,

            usdc_apr_usdc: QueryAprResponse::default(),
            eden_apr_usdc: QueryAprResponse::default(),
            usdc_apr_edenb: QueryAprResponse::default(),
            eden_apr_edenb: QueryAprResponse::default(),
            usdc_apr_eden: QueryAprResponse::default(),
            eden_apr_eden: QueryAprResponse::default(),
            edenb_apr_eden: QueryAprResponse::default(),
            usdc_apr_elys: QueryAprResponse::default(),
            eden_apr_elys: QueryAprResponse::default(),
            edenb_apr_elys: QueryAprResponse::default(),
            // // APR section
            // usdc_apr_usdc: querier
            //     .get_incentive_apr(
            //         EarnType::UsdcProgram as i32,
            //         ElysDenom::Usdc.as_str().to_string(),
            //     )
            //     .map_err(|_| {
            //         StdError::generic_err("an error occurred while getting usdc apr in usdc")
            //     })?,
            // eden_apr_usdc: querier
            //     .get_incentive_apr(
            //         EarnType::UsdcProgram as i32,
            //         ElysDenom::Eden.as_str().to_string(),
            //     )
            //     .map_err(|_| {
            //         StdError::generic_err("an error occurred while getting eden apr in usdc")
            //     })?,

            // usdc_apr_edenb: querier
            //     .get_incentive_apr(
            //         EarnType::EdenBProgram as i32,
            //         ElysDenom::Usdc.as_str().to_string(),
            //     )
            //     .map_err(|_| {
            //         StdError::generic_err("an error occurred while getting usdc apr in edenb")
            //     })?,
            // eden_apr_edenb: querier
            //     .get_incentive_apr(
            //         EarnType::EdenBProgram as i32,
            //         ElysDenom::Eden.as_str().to_string(),
            //     )
            //     .map_err(|_| {
            //         StdError::generic_err("an error occurred while getting eden apr in edenb")
            //     })?,

            // usdc_apr_eden: querier
            //     .get_incentive_apr(
            //         EarnType::EdenProgram as i32,
            //         ElysDenom::Usdc.as_str().to_string(),
            //     )
            //     .map_err(|_| {
            //         StdError::generic_err("an error occurred while getting usdc apr in eden")
            //     })?,
            // eden_apr_eden: querier
            //     .get_incentive_apr(
            //         EarnType::EdenProgram as i32,
            //         ElysDenom::Eden.as_str().to_string(),
            //     )
            //     .map_err(|_| {
            //         StdError::generic_err("an error occurred while getting eden apr in eden")
            //     })?,
            // edenb_apr_eden: querier
            //     .get_incentive_apr(
            //         EarnType::EdenProgram as i32,
            //         ElysDenom::EdenBoost.as_str().to_string(),
            //     )
            //     .map_err(|_| {
            //         StdError::generic_err("an error occurred while getting edenb apr in eden")
            //     })?,

            // usdc_apr_elys: querier
            //     .get_incentive_apr(
            //         EarnType::ElysProgram as i32,
            //         ElysDenom::Usdc.as_str().to_string(),
            //     )
            //     .map_err(|_| {
            //         StdError::generic_err("an error occurred while getting usdc apr in elys")
            //     })?,
            // eden_apr_elys: querier
            //     .get_incentive_apr(
            //         EarnType::ElysProgram as i32,
            //         ElysDenom::Eden.as_str().to_string(),
            //     )
            //     .map_err(|_| {
            //         StdError::generic_err("an error occurred while getting eden apr in elys")
            //     })?,
            // edenb_apr_elys: querier
            //     .get_incentive_apr(
            //         EarnType::ElysProgram as i32,
            //         ElysDenom::EdenBoost.as_str().to_string(),
            //     )
            //     .map_err(|_| {
            //         StdError::generic_err("an error occurred while getting edenb apr in elys")
            //     })?,
        })
    }

    pub fn generate_account_snapshot_for_address(
        &self,
        querier: &ElysQuerier,
        deps: &Deps<ElysQuery>,
        env: &Env,
        address: &String,
    ) -> StdResult<Option<AccountSnapshot>> {
        let account_balances = deps.querier.query_all_balances(address)?;
        let order_balances =
            self.get_all_orders(&deps.querier, &self.trade_shield_address, &address)?;
        // let staked_response = self.get_staked_assets(
        //     &deps,
        //     &address,
        //     self.uusdc_usd_price,
        //     self.uelys_price_in_uusdc,
        //     self.usdc_denom.to_owned(),
        //     self.usdc_base_denom.to_owned(),
        //     self.eden_decimal,
        //     self.usdc_apr_usdc.to_owned(),
        //     self.eden_apr_usdc.to_owned(),
        //     self.usdc_apr_edenb.to_owned(),
        //     self.eden_apr_edenb.to_owned(),
        //     self.usdc_apr_eden.to_owned(),
        //     self.eden_apr_eden.to_owned(),
        //     self.edenb_apr_eden.to_owned(),
        //     self.usdc_apr_elys.to_owned(),
        //     self.eden_apr_elys.to_owned(),
        //     self.edenb_apr_elys.to_owned(),
        // );
        // let rewards_response = self.get_rewards(&deps, address.clone())?;
        // let perpetual_response = match self.get_perpetuals(
        //     &deps,
        //     self.trade_shield_address.clone(),
        //     &self.usdc_denom,
        //     address.clone(),
        // ) {
        //     Ok(perpetual_response) => perpetual_response,
        //     Err(_) => PerpetualAssets {
        //         total_perpetual_asset_balance: DecCoin::new(Decimal256::zero(), &self.usdc_denom),
        //         perpetual_asset: vec![],
        //     },
        // };

        let new_part = self.create_new_part(
            &env.block,
            &querier,
            &self.expiration,
            account_balances,
            order_balances,
            StakedAssetsResponse {
                total_staked_balance: DecCoin::new(Decimal256::zero(), &self.usdc_denom),
                staked_assets: StakedAssets::default(),
            },
            GetRewardsResp {
                rewards: Reward::default(),
            },
            PerpetualAssets::default(),
            &self.usdc_denom,
        );

        return new_part;
    }

    pub fn create_new_part(
        &self,
        block: &BlockInfo,
        querier: &ElysQuerier<'_>,
        expiration: &Expiration,
        account_balances: Vec<Coin>,
        orders_balances: Vec<Coin>,
        staked_assets_resp: StakedAssetsResponse,
        rewards_response: GetRewardsResp,
        perpetual_response: PerpetualAssets,
        usdc_denom: &String,
    ) -> StdResult<Option<AccountSnapshot>> {
        let date = match expiration {
            Expiration::AtHeight(_) => Expiration::AtHeight(block.height),
            Expiration::AtTime(_) => Expiration::AtTime(block.time),
            Expiration::Never {} => panic!("never expire"),
        };

        let available_asset_balance: Vec<CoinValue> = account_balances
            .iter()
            .filter_map(
                |coin| match CoinValue::from_coin(coin, querier, usdc_denom) {
                    Ok(res) => Some(res),
                    Err(_) => None,
                },
            )
            .collect();

        let in_orders_asset_balance: Vec<CoinValue> = orders_balances
            .iter()
            .filter_map(
                |coin| match CoinValue::from_coin(coin, querier, usdc_denom) {
                    Ok(res) => Some(res),
                    Err(_) => None,
                },
            )
            .collect();

        let mut total_available_balance = DecCoin::new(Decimal256::zero(), usdc_denom);
        let mut total_in_orders_balance = DecCoin::new(Decimal256::zero(), usdc_denom);

        for balance in &available_asset_balance {
            total_available_balance.amount = total_available_balance
                .amount
                .checked_add(Decimal256::from(balance.amount_usdc.clone()))?
        }

        for balance in &in_orders_asset_balance {
            total_in_orders_balance.amount = total_in_orders_balance
                .amount
                .checked_add(Decimal256::from(balance.amount_usdc.clone()))?
        }

        let mut total_value_per_asset: HashMap<&String, CoinValue> = HashMap::new();

        for available in available_asset_balance.iter() {
            total_value_per_asset
                .entry(&available.denom)
                .and_modify(|e| {
                    e.amount_token += available.amount_token.clone();
                    e.amount_usdc += available.amount_usdc.clone();
                })
                .or_insert_with(|| available.clone());
        }

        for in_order in in_orders_asset_balance.iter() {
            total_value_per_asset
                .entry(&in_order.denom)
                .and_modify(|e| {
                    e.amount_token += in_order.amount_token.clone();
                    e.amount_usdc += in_order.amount_usdc.clone();
                })
                .or_insert_with(|| in_order.clone());
        }

        let total_value_per_asset: Vec<CoinValue> =
            total_value_per_asset.values().cloned().collect();

        let total_liquid_asset_balance = DecCoin::new(
            Decimal256::from(
                total_value_per_asset
                    .iter()
                    .map(|v| v.amount_usdc)
                    .fold(Decimal::zero(), |acc, item| acc + item),
            ),
            usdc_denom,
        );

        let reward = rewards_response.rewards;
        let portfolio_usd = DecCoin::new(
            total_liquid_asset_balance
                .amount
                .checked_add(Decimal256::from(
                    staked_assets_resp.total_staked_balance.amount.clone(),
                ))?
                .checked_add(
                    perpetual_response
                        .total_perpetual_asset_balance
                        .amount
                        .clone(),
                )?,
            usdc_denom,
        );
        let reward_usd: DecCoin =
            DecCoin::new(Decimal256::from(reward.clone().total_usd), usdc_denom);
        let total_balance = DecCoin::new(
            portfolio_usd.amount.checked_add(reward_usd.amount)?,
            usdc_denom,
        );

        // Adds the records all the time as we should return data to the FE even if it is 0 balanced.
        Ok(Some(AccountSnapshot {
            date,
            total_balance: TotalBalance {
                total_balance,
                portfolio_usd: portfolio_usd.clone(),
                reward_usd,
            },
            portfolio: Portfolio {
                balance_usd: portfolio_usd,
                liquid_assets_usd: total_liquid_asset_balance.clone(),
                staked_committed_usd: DecCoin::new(
                    Decimal256::from(staked_assets_resp.total_staked_balance.amount),
                    usdc_denom,
                ),
                liquidity_positions_usd: DecCoin::new(Decimal256::zero(), usdc_denom),
                leverage_lp_usd: DecCoin::new(Decimal256::zero(), usdc_denom),
                perpetual_assets_usd: perpetual_response.total_perpetual_asset_balance.clone(),
                usdc_earn_usd: DecCoin::new(Decimal256::zero(), usdc_denom),
                borrows_usd: DecCoin::new(Decimal256::zero(), usdc_denom),
            },
            reward,
            liquid_asset: LiquidAsset {
                total_liquid_asset_balance,
                total_available_balance,
                total_in_orders_balance,
                available_asset_balance,
                in_orders_asset_balance,
                total_value_per_asset,
            },
            staked_assets: staked_assets_resp.staked_assets,
            perpetual_assets: perpetual_response,
        }))
    }

    pub fn get_staked_assets(
        &self,
        deps: &Deps<ElysQuery>,
        address: &String,
        uusdc_usd_price: Decimal,
        uelys_price_in_uusdc: Decimal,
        usdc_denom: String,
        usdc_base_denom: String,
        eden_decimal: u64,
        usdc_apr_usdc: QueryAprResponse,
        eden_apr_usdc: QueryAprResponse,
        usdc_apr_edenb: QueryAprResponse,
        eden_apr_edenb: QueryAprResponse,
        usdc_apr_eden: QueryAprResponse,
        eden_apr_eden: QueryAprResponse,
        edenb_apr_eden: QueryAprResponse,
        usdc_apr_elys: QueryAprResponse,
        eden_apr_elys: QueryAprResponse,
        edenb_apr_elys: QueryAprResponse,
    ) -> StakedAssetsResponse {
        // create staked_assets variable that is a StakedAssets struct
        let mut staked_assets = StakedAssets::default();
        let mut total_balance = Decimal::zero();

        let usdc_details = get_usdc_earn_program_details(
            deps,
            Some(address.to_owned()),
            ElysDenom::Usdc.as_str().to_string(),
            usdc_denom.to_owned(),
            usdc_base_denom.to_owned(),
            uusdc_usd_price,
            uelys_price_in_uusdc,
            usdc_apr_usdc,
            eden_apr_usdc,
        )
        .unwrap();
        // usdc program
        let staked_asset_usdc = usdc_details.data.clone();
        total_balance = total_balance
            .checked_add(match staked_asset_usdc.clone() {
                UsdcEarnProgram {
                    staked: Some(r), ..
                } => r.usd_amount,
                _ => Decimal::zero(),
            })
            .unwrap();
        staked_assets.usdc_earn_program = staked_asset_usdc;

        // elys program
        let elys_details = get_elys_earn_program_details(
            deps,
            Some(address.to_owned()),
            ElysDenom::Elys.as_str().to_string(),
            usdc_denom.to_owned(),
            uusdc_usd_price,
            uelys_price_in_uusdc,
            usdc_apr_elys,
            eden_apr_elys,
            edenb_apr_elys,
        )
        .unwrap();
        let staked_asset_elys = elys_details.data;
        total_balance = total_balance
            .checked_add(match staked_asset_elys.clone() {
                ElysEarnProgram {
                    staked: Some(r), ..
                } => r.usd_amount,
                _ => Decimal::zero(),
            })
            .unwrap();
        staked_assets.elys_earn_program = staked_asset_elys;

        // eden program
        let eden_details = get_eden_earn_program_details(
            deps,
            Some(address.to_owned()),
            ElysDenom::Eden.as_str().to_string(),
            usdc_denom.to_owned(),
            uusdc_usd_price,
            uelys_price_in_uusdc,
            usdc_apr_eden,
            eden_apr_eden,
            edenb_apr_eden,
        )
        .unwrap();
        let staked_asset_eden = eden_details.data;
        total_balance = total_balance
            .checked_add(match staked_asset_eden.clone() {
                EdenEarnProgram {
                    staked: Some(r), ..
                } => r.usd_amount,
                _ => Decimal::zero(),
            })
            .unwrap();
        staked_assets.eden_earn_program = staked_asset_eden;

        let edenb_details = get_eden_boost_earn_program_details(
            deps,
            Some(address.to_owned()),
            ElysDenom::EdenBoost.as_str().to_string(),
            usdc_denom.to_owned(),
            uusdc_usd_price,
            uelys_price_in_uusdc,
            eden_decimal,
            usdc_apr_edenb,
            eden_apr_edenb,
        )
        .unwrap();
        let staked_asset_edenb = edenb_details.data;
        total_balance = total_balance
            .checked_add(match staked_asset_edenb.clone() {
                EdenBoostEarnProgram {
                    rewards: Some(r), ..
                } => r.iter().fold(Decimal::zero(), |acc, item| {
                    acc.checked_add(item.usd_amount.unwrap()).unwrap()
                }),
                _ => Decimal::zero(),
            })
            .unwrap();
        staked_assets.eden_boost_earn_program = staked_asset_edenb;

        StakedAssetsResponse {
            staked_assets,
            total_staked_balance: DecCoin::new(Decimal256::from(total_balance), usdc_denom),
        }
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

        for PerpetualOrder { collateral, .. } in perpetual_order.orders {
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
        address: String,
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
                    address,
                    pagination: None,
                },
            )
            .map_err(|_| StdError::generic_err("an error occurred while getting perpetuals"))?;
        let mut perpetual_vec: Vec<PerpetualAsset> = vec![];
        let querier = ElysQuerier::new(&deps.querier);

        for mtp in mtps {
            match PerpetualAsset::new(mtp, self.usdc_denom.to_owned(), &querier) {
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
            self.usdc_denom.to_owned(),
        );

        Ok(PerpetualAssets {
            total_perpetual_asset_balance,
            perpetual_asset: perpetual_vec,
        })
    }

    pub fn get_rewards(
        &self,
        deps: &Deps<ElysQuery>,
        address: String,
    ) -> StdResult<GetRewardsResp> {
        let querier = ElysQuerier::new(&deps.querier);
        let commitments = querier.get_commitments(address)?;

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
            .unwrap();

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
                        let usdc_rewards = Decimal::from_atomics(reward.amount, 0).unwrap();
                        rewards.usdc_usd = usdc_rewards.checked_mul(usdc_price).unwrap();
                        rewards.total_usd =
                            rewards.total_usd.checked_add(rewards.usdc_usd).unwrap();

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
                        let rewards_in_usdc = Decimal::from_atomics(amount.amount, 0).unwrap();
                        rewards.eden_usd = rewards_in_usdc.checked_mul(usdc_price).unwrap();
                        rewards.total_usd =
                            rewards.total_usd.checked_add(rewards.eden_usd).unwrap();
                        continue;
                    }

                    // uedenb - we don't value eden boost in usd.
                    if reward.denom == denom_uedenb {
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
                    let rewards_in_usdc = Decimal::from_atomics(amount.amount, 0).unwrap();
                    let rewards_in_usd = rewards_in_usdc.checked_mul(usdc_price).unwrap();

                    rewards.other_usd = rewards.other_usd.checked_add(rewards_in_usd).unwrap();
                    rewards.total_usd = rewards.total_usd.checked_add(rewards_in_usd).unwrap();
                }
            }
            None => {
                return Ok(GetRewardsResp {
                    rewards: AccountSnapshot::zero(&denom_uusdc).reward,
                });
            }
        }

        let resp = GetRewardsResp { rewards: rewards };
        Ok(resp)
    }
}