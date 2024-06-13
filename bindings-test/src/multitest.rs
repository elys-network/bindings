use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use anyhow::{bail, Error, Result as AnyResult};
#[allow(deprecated)]
use cosmwasm_std::{
    coin, coins,
    testing::{MockApi, MockStorage},
    to_json_binary, Addr, BankMsg, BlockInfo, Coin, Decimal, Empty, Int64, Querier, StdError,
    StdResult, Storage,
};
use cosmwasm_std::{Int128, SignedDecimal, Uint128};
use cw_multi_test::{App, AppResponse, BankKeeper, BankSudo, BasicAppBuilder, Module, WasmKeeper};
use cw_storage_plus::Item;
use elys_bindings::{
    msg_resp::{
        AmmSwapByDenomResponse, AmmSwapExactAmountInResp, MsgResponse, PerpetualCloseResponse,
        PerpetualOpenResponse,
    },
    query_resp::{
        AmmSwapEstimationByDenomResponse, AmmSwapEstimationResponse, AuthAddressesResponse,
        BalanceBorrowed, Commitments, Entry, EstakingRewardsResponse,
        LeveragelpIsWhitelistedResponse, LeveragelpParams, LeveragelpParamsResponse,
        LeveragelpStatusResponse, LeveragelpWhitelistResponse, MasterchefUserPendingRewardResponse,
        OracleAssetInfoResponse, PerpetualGetPositionsForAddressResponse, PerpetualMtpResponse,
        PerpetualOpenEstimationRawResponse, PerpetualQueryPositionsResponse, PoolApr,
        QueryAprResponse, QueryAprsResponse, QueryGetEntryAllResponse, QueryGetEntryResponse,
        QueryGetPriceResponse, QueryPoolAprsResponse, QueryShowCommitmentsResponse,
        QueryStableStakeAprResponse, QueryStakedPositionResponse, QueryUnstakedPositionResponse,
        QueryVestingInfoResponse, StableStakeParamsData, StableStakeParamsResp,
    },
    types::{
        BalanceAvailable, Mtp, OracleAssetInfo, PageResponse, Price, SwapAmountInRoute,
        SwapAmountOutRoute,
    },
    ElysMsg, ElysQuery,
};
use itertools::Itertools;
use std::cmp::max;

pub const PRICES: Item<Vec<Price>> = Item::new("prices");
pub const ASSET_INFO: Item<Vec<OracleAssetInfo>> = Item::new("asset_info");
pub const BLOCK_TIME: u64 = 5;
pub const PERPETUAL_OPENED_POSITION: Item<Vec<Mtp>> = Item::new("perpetual_opened_position");
pub const LAST_MODULE_USED: Item<Option<String>> = Item::new("last_module_used");
pub const ACCOUNT: Item<Vec<String>> = Item::new("account");

pub struct ElysModule {}

impl ElysModule {
    pub fn get_last_module(&self, store: &dyn Storage) -> StdResult<Option<String>> {
        LAST_MODULE_USED.load(store)
    }

    fn get_all_price(&self, store: &dyn Storage) -> StdResult<Vec<Price>> {
        PRICES.load(store)
    }

    pub fn set_prices(&self, store: &mut dyn Storage, prices: &Vec<Price>) -> StdResult<()> {
        PRICES.save(store, prices)
    }

    pub fn new_account(&self, store: &mut dyn Storage, addr: impl Into<String>) -> StdResult<()> {
        let mut accounts = ACCOUNT.load(store)?;
        let addr: String = addr.into();
        accounts.push(addr);
        ACCOUNT.save(store, &accounts)
    }

    pub fn new_price(&self, store: &mut dyn Storage, new_price: &Price) -> StdResult<()> {
        let mut prices = PRICES.load(store)?;
        for price in prices.iter_mut() {
            if price.asset == new_price.asset {
                *price = new_price.clone();
                return PRICES.save(store, &prices);
            }
        }
        prices.push(new_price.to_owned());
        PRICES.save(store, &prices)
    }
    pub fn set_asset_infos(
        &self,
        store: &mut dyn Storage,
        infos: &Vec<OracleAssetInfo>,
    ) -> StdResult<()> {
        ASSET_INFO.save(store, infos)
    }
    pub fn set_mtp(&self, store: &mut dyn Storage, mtps: &Vec<Mtp>) -> StdResult<()> {
        PERPETUAL_OPENED_POSITION.save(store, mtps)
    }

    pub fn get_balance(&self, store: &mut dyn Storage, mtps: &Vec<Mtp>) -> StdResult<()> {
        PERPETUAL_OPENED_POSITION.save(store, mtps)
    }
}

impl Module for ElysModule {
    type ExecT = ElysMsg;
    type QueryT = ElysQuery;
    type SudoT = Empty;

    fn query(
        &self,
        _api: &dyn cosmwasm_std::Api,
        storage: &dyn cosmwasm_std::Storage,
        _querier: &dyn cosmwasm_std::Querier,
        _block: &cosmwasm_std::BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        match request {
            ElysQuery::LeveragelpParams { .. } => {
                let resp = LeveragelpParamsResponse {
                    params: Some(LeveragelpParams {
                        leverage_max: Decimal::from_atomics(Uint128::new(10), 0).unwrap(),
                        max_open_positions: 5,
                        pool_open_threshold: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                        safety_factor: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                        whitelisting_enabled: true,
                        epoch_length: 10,
                    }),
                };
                Ok(to_json_binary(&resp)?)
            }

            ElysQuery::EstakingRewards { .. } => {
                // TODO: remove default instead proper mock
                Ok(to_json_binary(&EstakingRewardsResponse::default())?)
            }
            ElysQuery::LeveragelpQueryPositions { .. } => todo!("LeveragelpQueryPositions"),
            ElysQuery::LeveragelpQueryPositionsByPool { .. } => {
                todo!("LeveragelpQueryPositionsByPool")
            }

            ElysQuery::LeveragelpGetStatus { .. } => {
                let resp = LeveragelpStatusResponse {
                    open_position_count: 10,
                    lifetime_position_count: 100,
                };
                Ok(to_json_binary(&resp)?)
            }

            ElysQuery::LeveragelpQueryPositionsForAddress { .. } => {
                todo!("LeveragelpQueryPositionsForAddress")
            }

            ElysQuery::LeveragelpGetWhitelist { .. } => {
                let resp = LeveragelpWhitelistResponse {
                    whitelist: vec![],
                    pagination: None,
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::LeveragelpIsWhitelisted { .. } => {
                let resp = LeveragelpIsWhitelistedResponse {
                    is_whitelisted: false,
                    address: "".to_string(),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::LeveragelpPool { .. } => todo!("LeveragelpPool"),
            ElysQuery::LeveragelpPools { .. } => todo!("LeveragelpPools"),
            ElysQuery::LeveragelpPosition { .. } => todo!("LeveragelpPosition"),
            ElysQuery::LeveragelpCloseEstimation { .. } => todo!("LeveragelpCloseEstimation"),
            ElysQuery::LeveragelpOpenEstimation { .. } => todo!("LeveragelpOpenEstimation"),

            ElysQuery::AmmEarnMiningPoolAll { .. } => todo!("AmmEarnMiningPoolAll"),
            ElysQuery::AmmJoinPoolEstimation { .. } => todo!("AmmJoinPoolEstimation"),
            ElysQuery::AmmExitPoolEstimation { .. } => todo!("AmmJoinPoolEstimation"),
            ElysQuery::CommitmentAllValidators { .. } => todo!("CommitmentAllValidators"),
            ElysQuery::CommitmentDelegations { .. } => todo!("CommitmentDelegations"),
            ElysQuery::CommitmentDelegatorValidators { .. } => {
                todo!("CommitmentDelegatorValidators")
            }
            ElysQuery::CommitmentShowCommitments { .. } => {
                let resp = QueryShowCommitmentsResponse {
                    commitments: Commitments {
                        creator: "elys123".to_string(),
                        committed_tokens: Some(vec![]),
                        rewards_unclaimed: Some(vec![]),
                        claimed: Some(vec![]),
                        vesting_tokens: Some(vec![]),
                        rewards_by_elys_unclaimed: Some(vec![]),
                        rewards_by_eden_unclaimed: Some(vec![]),
                        rewards_by_edenb_unclaimed: Some(vec![]),
                        rewards_by_usdc_unclaimed: Some(vec![]),
                    },
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::CommitmentUnbondingDelegations { .. } => {
                todo!("CommitmentUnbondingDelegations")
            }
            ElysQuery::CommitmentVestingInfo { .. } => {
                let resp = QueryVestingInfoResponse {
                    vesting: BalanceAvailable {
                        amount: Uint128::new(100),
                        usd_amount: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                    },
                    vesting_details: Some(vec![]),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::OraclePriceAll { .. } => Ok(to_json_binary(&self.get_all_price(storage)?)?),
            ElysQuery::OracleAssetInfo { denom } => {
                let infos = ASSET_INFO.load(storage)?;
                let may_have_info = infos.iter().find(|asset| asset.denom == denom);

                match may_have_info {
                    Some(info) => Ok(to_json_binary(&OracleAssetInfoResponse {
                        asset_info: info.to_owned(),
                    })?),
                    None => Err(Error::new(StdError::not_found("asset denom"))),
                }
            }
            ElysQuery::AmmSwapEstimation {
                routes,
                token_in,
                discount,
            } => {
                let prices = &self.get_all_price(storage)?;
                let price_in = prices
                    .iter()
                    .find(|price| price.asset == token_in.denom)
                    .unwrap();
                let price_out = prices
                    .iter()
                    .find(|price| price.asset == routes[0].token_out_denom)
                    .unwrap();
                let spot_price = price_in.price / price_out.price;
                let token_out_amount =
                    (Decimal::from_atomics(token_in.amount, spot_price.decimal_places())?
                        * spot_price)
                        .atomics()
                        .u128();

                Ok(to_json_binary(&AmmSwapEstimationResponse {
                    spot_price,
                    token_out: coin(token_out_amount, &routes[0].token_out_denom),
                    discount,
                    swap_fee: SignedDecimal::from_str("0.1").unwrap(),
                    available_liquidity: coin(999999, &routes[0].token_out_denom),
                    slippage: Decimal::zero(),
                    weight_balance_ratio: Decimal::zero(),
                })?)
            }
            ElysQuery::AmmPool { .. } => todo!("not implemented"),
            ElysQuery::AmmPoolAll { .. } => todo!("not implemented"),
            ElysQuery::AmmSwapEstimationByDenom {
                amount,
                denom_in,
                denom_out,
                discount,
            } => {
                let prices = &self.get_all_price(storage)?;
                let price_in = prices.iter().find(|price| price.asset == denom_in).unwrap();
                let price_out = prices
                    .iter()
                    .find(|price| price.asset == denom_out)
                    .unwrap();
                let spot_price = price_in.price / price_out.price;

                let token_estimation = if amount.denom == denom_in {
                    coin(
                        (Decimal::from_atomics(amount.amount, spot_price.decimal_places())?
                            * &spot_price)
                            .atomics()
                            .u128(),
                        denom_out.clone(),
                    )
                } else {
                    coin(
                        (Decimal::from_atomics(amount.amount, spot_price.decimal_places())?
                            / &spot_price)
                            .atomics()
                            .u128(),
                        denom_in.clone(),
                    )
                };

                let (in_route, out_route) = if amount.denom == denom_in {
                    (
                        Some(vec![SwapAmountInRoute {
                            pool_id: 1,
                            token_out_denom: denom_out.clone(),
                        }]),
                        None,
                    )
                } else {
                    (
                        None,
                        Some(vec![SwapAmountOutRoute {
                            pool_id: 1,
                            token_in_denom: denom_in,
                        }]),
                    )
                };

                let resp = AmmSwapEstimationByDenomResponse {
                    in_route,
                    out_route,
                    spot_price,
                    amount: token_estimation,
                    discount: SignedDecimal::try_from(discount).unwrap(),
                    swap_fee: SignedDecimal::from_str("0.1").unwrap(),
                    available_liquidity: coin(999999, denom_out),
                    weight_balance_ratio: SignedDecimal::zero(),
                    price_impact: SignedDecimal::zero(),
                    slippage: Decimal::zero(),
                };

                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::PerpetualMtp { address, id } => {
                let mtps = PERPETUAL_OPENED_POSITION.load(storage)?;
                if let Some(mtp) = mtps
                    .iter()
                    .find(|mtp| mtp.id == id && mtp.address == address)
                    .cloned()
                {
                    Ok(to_json_binary(&PerpetualMtpResponse { mtp: Some(mtp) })?)
                } else {
                    return Err(Error::new(StdError::not_found(
                        "perpetual trading position",
                    )));
                }
            }
            ElysQuery::PerpetualQueryPositions { pagination } => {
                let mtps = PERPETUAL_OPENED_POSITION.load(storage)?;
                let (mtps, page_resp) = pagination.filter(mtps)?;
                Ok(to_json_binary(&PerpetualQueryPositionsResponse {
                    mtps: Some(mtps),
                    pagination: page_resp,
                })?)
            }
            ElysQuery::AmmBalance { .. } => {
                let resp = BalanceAvailable {
                    amount: Uint128::new(100),
                    usd_amount: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::PerpetualOpenEstimation {
                position,
                leverage,
                trading_asset,
                collateral,
                take_profit_price,
                discount,
            } => {
                return Ok(to_json_binary(&PerpetualOpenEstimationRawResponse {
                    position,
                    min_collateral: coin(0, &collateral.denom),
                    available_liquidity: coin(99999999, &trading_asset),
                    leverage: leverage.to_string(),
                    collateral,
                    trading_asset,
                    discount: discount.to_string(),
                    valid_collateral: Some(true),
                    position_size: coin(1, "btc"),
                    swap_fee: Decimal::zero().to_string(),
                    open_price: Decimal::zero().to_string(),
                    take_profit_price: take_profit_price.to_string(),
                    liquidation_price: Decimal::zero().to_string(),
                    estimated_pnl: Int128::zero(),
                    estimated_pnl_denom: "uelys".to_string(),
                    slippage: Decimal::zero().to_string(),
                    weight_balance_ratio: Decimal::zero().to_string(),
                    borrow_interest_rate: Decimal::zero().to_string(),
                    funding_rate: Decimal::zero().to_string(),
                    price_impact: Decimal::zero().to_string(),
                })?)
            }
            ElysQuery::AssetProfileEntryAll { .. } => {
                let asset_info = ASSET_INFO.load(storage)?;
                let entries: Vec<Entry> = asset_info
                    .iter()
                    .map(|info| Entry {
                        base_denom: info.denom.clone(),
                        decimals: info.decimal,
                        denom: info.denom.clone(),
                        path: "".to_string(),
                        ibc_channel_id: "".to_string(),
                        ibc_counterparty_channel_id: "".to_string(),
                        display_name: "".to_string(),
                        display_symbol: "".to_string(),
                        external_symbol: "".to_string(),
                        unit_denom: "".to_string(),
                        authority: "".to_string(),
                        commit_enabled: true,
                        withdraw_enabled: true,
                        network: "".to_string(),
                        address: "".to_string(),
                        transfer_limit: "".to_string(),
                        permissions: vec![],
                        ibc_counterparty_denom: "".to_string(),
                        ibc_counterparty_chain_id: "".to_string(),
                    })
                    .collect();
                Ok(to_json_binary(&QueryGetEntryAllResponse {
                    entry: Some(entries.clone()),
                    pagination: PageResponse {
                        next_key: None,
                        total: Some(entries.len() as u64),
                    },
                })?)
            }
            ElysQuery::AssetProfileEntry { base_denom } => {
                let resp = match base_denom.as_str() {
                    "uusdc" => QueryGetEntryResponse {
                        entry: Entry {
                            address: "".to_string(),
                            authority: "elys10d07y265gmmuvt4z0w9aw880jnsr700j6z2zm3".to_string(),
                            base_denom: "uusdc".to_string(),
                            commit_enabled: true,
                            decimals: 6,
                            denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
                            display_name: "USDC".to_string(),
                            display_symbol: "uUSDC".to_string(),
                            external_symbol: "uUSDC".to_string(),
                            ibc_channel_id: "channel-12".to_string(),
                            ibc_counterparty_chain_id: "".to_string(),
                            ibc_counterparty_channel_id: "channel-19".to_string(),
                            ibc_counterparty_denom: "".to_string(),
                            network: "".to_string(),
                            path: "transfer/channel-12".to_string(),
                            permissions: vec![],
                            transfer_limit: "".to_string(),
                            unit_denom: "uusdc".to_string(),
                            withdraw_enabled: true,
                        },
                    },
                    "ueden" => QueryGetEntryResponse {
                        entry: Entry {
                            address: "".to_string(),
                            authority: "elys10d07y265gmmuvt4z0w9aw880jnsr700j6z2zm3".to_string(),
                            base_denom: "ueden".to_string(),
                            commit_enabled: true,
                            decimals: 6,
                            denom: "ueden".to_string(),
                            display_name: "EDEN".to_string(),
                            display_symbol: "".to_string(),
                            external_symbol: "".to_string(),
                            ibc_channel_id: "".to_string(),
                            ibc_counterparty_chain_id: "".to_string(),
                            ibc_counterparty_channel_id: "".to_string(),
                            ibc_counterparty_denom: "".to_string(),
                            network: "".to_string(),
                            path: "".to_string(),
                            permissions: vec![],
                            transfer_limit: "".to_string(),
                            unit_denom: "".to_string(),
                            withdraw_enabled: true,
                        },
                    },
                    "uelys" => QueryGetEntryResponse {
                        entry: Entry {
                            address: "".to_string(),
                            authority: "elys10d07y265gmmuvt4z0w9aw880jnsr700j6z2zm3".to_string(),
                            base_denom: "uelys".to_string(),
                            commit_enabled: true,
                            decimals: 6,
                            denom: "uelys".to_string(),
                            display_name: "ELYS".to_string(),
                            display_symbol: "".to_string(),
                            external_symbol: "".to_string(),
                            ibc_channel_id: "".to_string(),
                            ibc_counterparty_chain_id: "".to_string(),
                            ibc_counterparty_channel_id: "".to_string(),
                            ibc_counterparty_denom: "".to_string(),
                            network: "".to_string(),
                            path: "".to_string(),
                            permissions: vec![],
                            transfer_limit: "".to_string(),
                            unit_denom: "".to_string(),
                            withdraw_enabled: true,
                        },
                    },
                    _ => return Err(StdError::not_found(base_denom).into()),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::PerpetualGetPositionsForAddress { address, .. } => {
                let all_mtps = PERPETUAL_OPENED_POSITION.load(storage)?;

                let user_mtps: Vec<Mtp> = all_mtps
                    .into_iter()
                    .filter(|mtp| mtp.address == address)
                    .collect();

                Ok(to_json_binary(&PerpetualGetPositionsForAddressResponse {
                    mtps: user_mtps,
                    pagination: PageResponse::empty(false),
                })?)
            }
            ElysQuery::AuthAddresses { .. } => {
                let addresses = ACCOUNT.load(storage)?;
                let res = AuthAddressesResponse {
                    addresses,
                    pagination: PageResponse::empty(false),
                };
                Ok(to_json_binary(&res)?)
            }
            ElysQuery::IncentiveApr { .. } => {
                let resp = QueryAprResponse {
                    apr: Uint128::zero(),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::IncentiveAprs { .. } => {
                let resp = QueryAprsResponse {
                    usdc_apr_usdc: Uint128::zero(),
                    eden_apr_usdc: Uint128::zero(),
                    usdc_apr_edenb: Uint128::zero(),
                    eden_apr_edenb: Uint128::zero(),
                    usdc_apr_eden: Uint128::zero(),
                    eden_apr_eden: Uint128::zero(),
                    edenb_apr_eden: Uint128::zero(),
                    usdc_apr_elys: Uint128::zero(),
                    eden_apr_elys: Uint128::zero(),
                    edenb_apr_elys: Uint128::zero(),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::OraclePrice { asset, .. } => {
                if asset.as_str() == "USDC" {
                    let resp = QueryGetPriceResponse {
                        price: Price {
                            asset: asset.clone(),
                            price: Decimal::one(),
                            source: asset.clone(),
                            provider: asset.clone(),
                            timestamp: 0,
                            block_height: 0,
                        },
                    };
                    return Ok(to_json_binary(&resp)?);
                }

                let prices = PRICES.load(storage).unwrap();
                let info = ASSET_INFO.load(storage).unwrap();

                let asset_info = info
                    .iter()
                    .find(|i| i.band_ticker == asset)
                    .cloned()
                    .unwrap();

                let price = prices
                    .iter()
                    .find(|price| price.asset == asset_info.denom)
                    .cloned()
                    .unwrap()
                    .price;

                let resp = QueryGetPriceResponse {
                    price: Price {
                        asset: asset.clone(),
                        price,
                        source: asset.clone(),
                        provider: asset.clone(),
                        timestamp: 0,
                        block_height: 0,
                    },
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::CommitmentStakedBalanceOfDenom { .. } => {
                // This is returning the same staked balance for each staking program (Usdc program, eden program, elys program, etc.).
                let resp = BalanceAvailable {
                    amount: Uint128::new(100),
                    usd_amount: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::AmmPriceByDenom { token_in, .. } => {
                let prices = &self.get_all_price(storage)?;
                let price_in = prices
                    .iter()
                    .find(|price| price.asset == token_in.denom)
                    .unwrap();
                let price_out = prices
                    .iter()
                    .find(|price| price.asset == "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string())
                    .unwrap();
                let spot_price = price_in.price / price_out.price;

                let resp = spot_price;
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::CommitmentStakedPositions { .. } => {
                let resp = QueryStakedPositionResponse {
                    staked_position: None,
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::CommitmentUnStakedPositions { .. } => {
                let resp = QueryUnstakedPositionResponse {
                    unstaked_position: None,
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::StableStakeParams { .. } => {
                let resp = StableStakeParamsResp {
                    params: StableStakeParamsData {
                        deposit_denom: String::from("uusdc"),
                        redemption_rate: Decimal::new(Uint128::from(1u128)),
                        epoch_length: 10i64,
                        interest_rate: Decimal::zero(),
                        interest_rate_max: Decimal::zero(),
                        interest_rate_min: Decimal::zero(),
                        interest_rate_increase: Decimal::zero(),
                        interest_rate_decrease: Decimal::zero(),
                        health_gain_factor: Decimal::zero(),
                        total_value: Uint128::zero(),
                    },
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::StableStakeBalanceOfBorrow { .. } => {
                let resp = BalanceBorrowed {
                    usd_amount: Decimal::zero(),
                    percentage: Decimal::zero(),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::MasterchefParams {} => todo!("MasterchefParams"),
            ElysQuery::MasterchefPoolInfo { .. } => todo!("MasterchefPool"),
            ElysQuery::MasterchefUserPendingReward { .. } => {
                // TODO: remove default instead proper mock
                Ok(to_json_binary(
                    &MasterchefUserPendingRewardResponse::default(),
                )?)
            }
            ElysQuery::MasterchefPoolAprs { pool_ids } => {
                let resp = QueryPoolAprsResponse {
                    data: pool_ids
                        .iter()
                        .map(|v| PoolApr {
                            pool_id: *v,
                            ..Default::default()
                        })
                        .collect_vec(),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::MasterchefStableStakeApr { .. } => {
                // TODO: remove default instead proper mock
                Ok(to_json_binary(&QueryStableStakeAprResponse {
                    apr: Int128::default(),
                })?)
            }
            ElysQuery::CommitmentNumberOfCommitments {} => todo!("CommitmentNumberOfCommitments"),
        }
    }

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        _sender: cosmwasm_std::Addr,
        msg: Self::ExecT,
    ) -> AnyResult<cw_multi_test::AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        match msg {
            ElysMsg::EstakingWithdrawElysStakingRewards { .. } => {
                todo!("EstakingWithdrawElysStakingRewards")
            }
            ElysMsg::EstakingWithdrawReward { .. } => todo!("EstakingWithdrawReward"),
            ElysMsg::AmmSwapExactAmountIn {
                sender,
                routes,
                token_in,
                token_out_min_amount,
                discount,
                recipient,
            } => {
                LAST_MODULE_USED.save(storage, &Some("AmmSwap".to_string()))?;
                let route = routes[0].clone();
                let prices = self.get_all_price(storage)?;
                let price_in = prices.iter().find(|p| p.asset == token_in.denom).unwrap();
                let price_out = prices
                    .iter()
                    .find(|p| p.asset == route.token_out_denom)
                    .unwrap();

                let mint_amount = coins(
                    (token_in.amount * (price_in.price / price_out.price)).u128(),
                    route.token_out_denom,
                );

                if (mint_amount[0].amount.u128() as i128) < (token_out_min_amount.i128()) {
                    return Err(Error::new(StdError::generic_err("not enough token")));
                }

                let data = to_json_binary(&AmmSwapExactAmountInResp {
                    token_out_amount: Int64::new(mint_amount[0].amount.u128() as i64),
                    discount,
                    swap_fee: Decimal::from_str("0.1").unwrap(),
                    recipient: recipient.clone(),
                })?;

                let mint = BankSudo::Mint {
                    to_address: recipient,
                    amount: mint_amount.clone(),
                };

                let burn = BankMsg::Burn {
                    amount: vec![token_in],
                };
                router
                    .execute(
                        api,
                        storage,
                        block,
                        Addr::unchecked(sender.clone()),
                        burn.into(),
                    )
                    .unwrap();
                router.sudo(api, storage, block, mint.into()).unwrap();

                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }

            ElysMsg::PerpetualOpen {
                creator,
                collateral,
                position,
                leverage,
                take_profit_price,
                trading_asset,
                owner,
            } => {
                LAST_MODULE_USED.save(storage, &Some("PerpetualOpen".to_string()))?;
                let mut mtp_vec = PERPETUAL_OPENED_POSITION.load(storage)?;

                let mtp_id: u64 = match mtp_vec.iter().max_by_key(|mtp| mtp.id) {
                    Some(mtp) => mtp.id + 1,
                    None => 0,
                };
                let collaterals = vec![collateral.clone()];
                let custody = (leverage.clone()
                    * SignedDecimal::from_atomics(
                        Int128::new(collateral.amount.u128() as i128),
                        0,
                    )
                    .unwrap())
                .floor()
                .atomics();
                let mtp: Mtp = Mtp {
                    address: owner,
                    liabilities: Int128::zero(),
                    take_profit_liabilities: Int128::zero(),
                    mtp_health: SignedDecimal::one(),
                    position,
                    id: mtp_id,
                    amm_pool_id: 0,
                    consolidate_leverage: SignedDecimal::zero(),
                    sum_collateral: Int128::zero(),
                    take_profit_price,
                    borrow_interest_paid_collateral: Int128::zero(),
                    borrow_interest_paid_custody: Int128::zero(),
                    borrow_interest_unpaid_collateral: Int128::zero(),
                    collateral_asset: collateral.denom,
                    collateral: Int128::new((collateral.amount.u128()) as i128),
                    custody,
                    custody_asset: "".to_string(),
                    funding_fee_paid_collateral: Int128::zero(),
                    funding_fee_paid_custody: Int128::zero(),
                    funding_fee_received_collateral: Int128::zero(),
                    funding_fee_received_custody: Int128::zero(),
                    leverage: SignedDecimal::try_from(leverage)?,
                    liabilities_asset: "".to_string(),
                    open_price: SignedDecimal::zero(),
                    take_profit_borrow_rate: SignedDecimal::zero(),
                    take_profit_custody: Int128::zero(),
                    trading_asset,
                };

                let msg_resp = PerpetualOpenResponse { id: mtp.id };

                let resp = AppResponse {
                    events: vec![],
                    data: Some(to_json_binary(&msg_resp)?),
                };

                mtp_vec.push(mtp);
                PERPETUAL_OPENED_POSITION.save(storage, &mtp_vec).unwrap();

                let burn_msg = BankMsg::Burn {
                    amount: collaterals,
                };
                router
                    .execute(
                        api,
                        storage,
                        block,
                        Addr::unchecked(creator),
                        burn_msg.into(),
                    )
                    .unwrap();

                Ok(resp)
            }

            ElysMsg::PerpetualClose {
                id, amount, owner, ..
            } => {
                LAST_MODULE_USED.save(storage, &Some("PerpetualClose".to_string()))?;
                let mtps: Vec<Mtp> = PERPETUAL_OPENED_POSITION.load(storage)?;

                let mut mtp = mtps
                    .iter()
                    .find(|mtp| mtp.address.as_str() == owner.as_str() && mtp.id == id)
                    .cloned()
                    .expect("mtp not found");

                if mtp.custody < amount {
                    panic!(
                        "amount: [{}] > custody: [{}]",
                        amount.i128(),
                        mtp.custody.i128()
                    );
                }

                let data = Some(to_json_binary(&PerpetualCloseResponse { id, amount })?);
                let resp = AppResponse {
                    events: vec![],
                    data,
                };

                let mut mtps: Vec<Mtp> = mtps
                    .iter()
                    .filter(|mtp| !(mtp.address.as_str() == owner.as_str() && mtp.id == id))
                    .cloned()
                    .collect();

                if mtp.custody > amount {
                    mtp.custody = mtp.custody.checked_sub(amount.clone())?;
                    mtps.push(mtp);
                }

                PERPETUAL_OPENED_POSITION.save(storage, &mtps)?;
                Ok(resp)
            }
            ElysMsg::AmmSwapByDenom {
                sender,
                amount,
                min_amount,
                in_denom,
                out_denom,
                discount,
                recipient,
                ..
            } => {
                LAST_MODULE_USED.save(storage, &Some("AmmSwapByDenom".to_string()))?;
                let prices = PRICES.load(storage)?;

                let price_in = prices.iter().find(|p| p.asset == in_denom).unwrap();
                let price_out = prices.iter().find(|p| p.asset == out_denom).unwrap();

                let spot_price = price_in.price / price_out.price;

                let mint_amount = coins((amount.amount * spot_price).u128(), &out_denom);

                if mint_amount[0].amount.u128() <= min_amount.amount.u128() {
                    return Err(Error::new(StdError::generic_err("not enough token")));
                }

                let data = to_json_binary(&AmmSwapByDenomResponse {
                    amount: mint_amount[0].clone(),
                    in_route: Some(vec![SwapAmountInRoute::new(1, out_denom)]),
                    out_route: None,
                    spot_price,
                    discount,
                    swap_fee: Decimal::from_str("0.1").unwrap(),
                    recipient,
                })?;

                let mint = BankSudo::Mint {
                    to_address: sender.clone(),
                    amount: mint_amount.clone(),
                };

                let burn = BankMsg::Burn {
                    amount: vec![amount],
                };
                router
                    .execute(
                        api,
                        storage,
                        block,
                        Addr::unchecked(sender.clone()),
                        burn.into(),
                    )
                    .unwrap();
                router.sudo(api, storage, block, mint.into()).unwrap();

                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::CommitmentStake { .. } => {
                LAST_MODULE_USED.save(storage, &Some("Commitment".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::CommitmentUnstake { .. } => {
                LAST_MODULE_USED.save(storage, &Some("Commitment".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::IncentiveBeginRedelegate { .. } => {
                LAST_MODULE_USED.save(storage, &Some("Incentive".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::IncentiveCancelUnbondingDelegation { .. } => {
                LAST_MODULE_USED.save(storage, &Some("Incentive".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::CommitmentVest { .. } => {
                LAST_MODULE_USED.save(storage, &Some("Commitment".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::CommitmentCancelVest { .. } => {
                LAST_MODULE_USED.save(storage, &Some("Commitment".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::CommitmentClaimVesting { .. } => {
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::AmmJoinPool { .. } => {
                LAST_MODULE_USED.save(storage, &Some("AmmJoin".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::AmmExitPool { .. } => {
                LAST_MODULE_USED.save(storage, &Some("AmmExit".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            // TODO @josefleventon
            ElysMsg::LeveragelpOpen { .. } => {
                LAST_MODULE_USED.save(storage, &Some("LeveragelpOpen".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::LeveragelpClose { .. } => {
                LAST_MODULE_USED.save(storage, &Some("LeveragelpClose".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::LeveragelpUpdateStopLoss { .. } => {
                LAST_MODULE_USED.save(storage, &Some("LeveragelpUpdateStopLoss".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::MasterchefClaimRewards { .. } => {
                LAST_MODULE_USED.save(storage, &Some("MasterchefClaimRewards".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<cw_multi_test::AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        bail!("sudo is not implemented for ElysMsg")
    }
}

pub type ElysAppWrapped =
    App<BankKeeper, MockApi, MockStorage, ElysModule, WasmKeeper<ElysMsg, ElysQuery>>;

pub struct ElysApp(ElysAppWrapped);

impl Deref for ElysApp {
    type Target = ElysAppWrapped;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElysApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Querier for ElysApp {
    fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
        self.0.raw_query(bin_request)
    }
}

impl Default for ElysApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ElysApp {
    pub fn new_with_wallets(wallets: Vec<(&str, Vec<Coin>)>) -> Self {
        let mut addresses: Vec<String> = vec![];
        Self(
            BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
                .with_custom(ElysModule {})
                .build(|roouter, _, storage| {
                    for (wallet_owner, wallet_contenent) in wallets {
                        roouter
                            .bank
                            .init_balance(storage, &Addr::unchecked(wallet_owner), wallet_contenent)
                            .unwrap();
                        addresses.push(wallet_owner.to_owned())
                    }
                    ACCOUNT.save(storage, &addresses).unwrap();
                    PERPETUAL_OPENED_POSITION.save(storage, &vec![]).unwrap();
                    ASSET_INFO.save(storage, &vec![]).unwrap();
                    PRICES.save(storage, &vec![]).unwrap();
                    LAST_MODULE_USED.save(storage, &None).unwrap();
                }),
        )
    }

    pub fn new() -> Self {
        Self(
            BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
                .with_custom(ElysModule {})
                .build(|_roouter, _, storage| {
                    PERPETUAL_OPENED_POSITION.save(storage, &vec![]).unwrap();
                    ASSET_INFO.save(storage, &vec![]).unwrap();
                    PRICES.save(storage, &vec![]).unwrap();
                    LAST_MODULE_USED.save(storage, &None).unwrap();
                }),
        )
    }
    pub fn block_info(&self) -> BlockInfo {
        self.0.block_info()
    }
    pub fn advance_blocks(&mut self, blocks: u64) {
        self.update_block(|block| {
            block.time = block.time.plus_seconds(BLOCK_TIME * blocks);
            block.height += blocks;
        });
    }

    /// This advances BlockInfo by given number of seconds.
    /// It does not do any callbacks, but keeps the ratio of seconds/block
    pub fn advance_seconds(&mut self, seconds: u64) {
        self.update_block(|block| {
            block.time = block.time.plus_seconds(seconds);
            block.height += max(1, seconds / BLOCK_TIME);
        });
    }

    /// Simple iterator when you don't care too much about the details and just want to
    /// simulate forward motion.
    pub fn next_block(&mut self) {
        self.advance_blocks(1)
    }
}
