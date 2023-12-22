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
use cosmwasm_std::{Int128, Uint128};
use cw_multi_test::{App, AppResponse, BankKeeper, BankSudo, BasicAppBuilder, Module, WasmKeeper};
use cw_storage_plus::Item;
use elys_bindings::{
    msg_resp::{
        AmmSwapByDenomResponse, AmmSwapExactAmountInResp, MarginCloseResponse, MarginOpenResponse,
        MsgResponse,
    },
    query_resp::{
        AmmSwapEstimationByDenomResponse, AmmSwapEstimationResponse, AuthAccountsResponse, Entry,
        MarginGetPositionsForAddressResponse, MarginMtpResponse, MarginOpenEstimationResponse,
        MarginQueryPositionsResponse, QueryGetEntryResponse,
    },
    types::{
        BalanceAvailable, BaseAccount, Mtp, OracleAssetInfo, PageResponse, Price, PublicKey, Sum,
        SwapAmountInRoute, SwapAmountOutRoute,
    },
    ElysMsg, ElysQuery,
};
use std::cmp::max;

pub const PRICES: Item<Vec<Price>> = Item::new("prices");
pub const ASSET_INFO: Item<Vec<OracleAssetInfo>> = Item::new("asset_info");
pub const BLOCK_TIME: u64 = 5;
pub const MARGIN_OPENED_POSITION: Item<Vec<Mtp>> = Item::new("margin_opened_position");
pub const LAST_MODULE_USED: Item<Option<String>> = Item::new("last_module_used");
pub const ACCOUNT: Item<Vec<BaseAccount>> = Item::new("account");

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
        let mut accounts: Vec<BaseAccount> = ACCOUNT.load(store)?;
        let addr: String = addr.into();

        accounts.push(BaseAccount {
            pub_key: PublicKey::set(Sum::Ed25519(to_json_binary(&addr)?)),
            address: addr,
            account_number: 0,
            sequence: 0,
        });

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
        MARGIN_OPENED_POSITION.save(store, mtps)
    }

    pub fn get_balance(&self, store: &mut dyn Storage, mtps: &Vec<Mtp>) -> StdResult<()> {
        MARGIN_OPENED_POSITION.save(store, mtps)
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
            ElysQuery::OraclePriceAll { .. } => Ok(to_json_binary(&self.get_all_price(storage)?)?),
            ElysQuery::OracleAssetInfo { denom } => {
                let infos = ASSET_INFO.load(storage)?;
                let may_have_info = infos.iter().find(|asset| asset.denom == denom);

                match may_have_info {
                    Some(info) => Ok(to_json_binary(info)?),
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
                    swap_fee: Decimal::from_str("0.1").unwrap(),
                    available_liquidity: coin(999999, &routes[0].token_out_denom),
                })?)
            }
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
                    discount,
                    swap_fee: Decimal::from_str("0.1").unwrap(),
                    available_liquidity: coin(999999, denom_out),
                };

                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::MarginMtp { address, id } => {
                let mtps = MARGIN_OPENED_POSITION.load(storage)?;
                if let Some(mtp) = mtps
                    .iter()
                    .find(|mtp| mtp.id == id && mtp.address == address)
                    .cloned()
                {
                    Ok(to_json_binary(&MarginMtpResponse { mtp: Some(mtp) })?)
                } else {
                    return Err(Error::new(StdError::not_found("margin trading position")));
                }
            }
            ElysQuery::MarginQueryPositions { pagination } => {
                let mtps = MARGIN_OPENED_POSITION.load(storage)?;
                let (mtps, page_resp) = pagination.filter(mtps)?;
                Ok(to_json_binary(&MarginQueryPositionsResponse {
                    mtps: Some(mtps),
                    pagination: page_resp,
                })?)
            }
            ElysQuery::AuthAccounts { pagination } => {
                let acc = ACCOUNT.load(storage)?;
                let (accounts, pagination) = pagination.filter(acc)?;
                let resp = AuthAccountsResponse {
                    accounts,
                    pagination,
                };

                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::AmmBalance { .. } => {
                let resp = BalanceAvailable {
                    amount: Uint128::new(100),
                    usd_amount: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::MarginOpenEstimation {
                position,
                leverage,
                trading_asset,
                collateral,
                take_profit_price,
                discount,
            } => {
                return Ok(to_json_binary(&MarginOpenEstimationResponse {
                    position,
                    min_collateral: coin(0, &collateral.denom),
                    available_liquidity: coin(99999999, &trading_asset),
                    leverage,
                    collateral,
                    trading_asset,
                    discount,
                    valid_collateral: true,
                    position_size: coin(1, "btc"),
                    swap_fee: Decimal::zero(),
                    open_price: Decimal::zero(),
                    take_profit_price,
                    liquidation_price: Decimal::zero(),
                    estimated_pnl: Int128::zero(),
                })?)
            }
            ElysQuery::AssetProfileEntry { .. } => {
                let resp = QueryGetEntryResponse {
                    entry: Entry {
                        base_denom: "uusdc".to_string(),
                        decimals: 6,
                        denom: "uusdc".to_string(),
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
                    },
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::MarginGetPositionsForAddress { address, .. } => {
                let all_mtps = MARGIN_OPENED_POSITION.load(storage)?;

                let user_mtps: Vec<Mtp> = all_mtps
                    .into_iter()
                    .filter(|mtp| mtp.address == address)
                    .collect();

                Ok(to_json_binary(&MarginGetPositionsForAddressResponse {
                    mtps: user_mtps,
                    pagination: PageResponse::empty(false),
                })?)
            }
        }
    }

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        sender: cosmwasm_std::Addr,
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

            ElysMsg::MarginOpen {
                creator,
                collateral,
                position,
                leverage,
                take_profit_price,
                ..
            } => {
                LAST_MODULE_USED.save(storage, &Some("MarginOpen".to_string()))?;
                let mut mtp_vec = MARGIN_OPENED_POSITION.load(storage)?;

                let mtp_id: u64 = match mtp_vec.iter().max_by_key(|mtp| mtp.id) {
                    Some(mtp) => mtp.id + 1,
                    None => 0,
                };
                let collaterals = vec![collateral.clone()];

                let mtp: Mtp = Mtp {
                    address: creator,
                    liabilities: Int128::zero(),
                    take_profit_liabilities: Int128::zero(),
                    mtp_health: Decimal::one(),
                    position,
                    id: mtp_id,
                    amm_pool_id: 0,
                    consolidate_leverage: Decimal::zero(),
                    sum_collateral: Int128::zero(),
                    take_profit_price,
                    borrow_interest_paid_collateral: Int128::zero(),
                    borrow_interest_paid_custody: Int128::zero(),
                    borrow_interest_unpaid_collateral: Int128::zero(),
                    collateral_asset: collateral.denom,
                    collateral: Int128::new((collateral.amount.u128()) as i128),
                    custody: Int128::zero(),
                    custody_asset: "".to_string(),
                    funding_fee_paid_collateral: Int128::zero(),
                    funding_fee_paid_custody: Int128::zero(),
                    funding_fee_received_collateral: Int128::zero(),
                    funding_fee_received_custody: Int128::zero(),
                    leverage,
                    liabilities_asset: "".to_string(),
                    open_price: Decimal::zero(),
                    take_profit_borrow_rate: take_profit_price,
                    take_profit_custody: Int128::zero(),
                    trading_asset: "".to_string(),
                };

                let msg_resp = MarginOpenResponse { id: mtp.id };

                let resp = AppResponse {
                    events: vec![],
                    data: Some(to_json_binary(&msg_resp)?),
                };

                mtp_vec.push(mtp);

                let burn_msg = BankMsg::Burn {
                    amount: collaterals,
                };
                router
                    .execute(api, storage, block, sender, burn_msg.into())
                    .unwrap();

                Ok(resp)
            }

            ElysMsg::MarginClose { id, amount, .. } => {
                LAST_MODULE_USED.save(storage, &Some("MarginClose".to_string()))?;
                let orders: Vec<Mtp> = MARGIN_OPENED_POSITION.load(storage)?;

                let new_orders: Vec<Mtp> =
                    orders.into_iter().filter(|order| order.id != id).collect();

                MARGIN_OPENED_POSITION.save(storage, &new_orders)?;

                let data = Some(to_json_binary(&MarginCloseResponse { id, amount })?);

                Ok(AppResponse {
                    events: vec![],
                    data,
                })
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
            ElysMsg::IncentiveWithdrawRewards { .. } => {
                LAST_MODULE_USED.save(storage, &Some("Incentive".to_string()))?;
                let data = to_json_binary(&MsgResponse {
                    result: "Ok".to_string(),
                })?;
                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
            ElysMsg::IncentiveWithdrawValidatorCommission { .. } => {
                LAST_MODULE_USED.save(storage, &Some("Incentive".to_string()))?;
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
        let mut accounts: Vec<BaseAccount> = vec![];
        Self(
            BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
                .with_custom(ElysModule {})
                .build(|roouter, _, storage| {
                    for (wallet_owner, wallet_contenent) in wallets {
                        roouter
                            .bank
                            .init_balance(storage, &Addr::unchecked(wallet_owner), wallet_contenent)
                            .unwrap();
                        accounts.push(BaseAccount {
                            address: wallet_owner.to_owned(),
                            pub_key: PublicKey::set(Sum::Ed25519(
                                to_json_binary(wallet_owner).unwrap(),
                            )),
                            account_number: 0,
                            sequence: 0,
                        })
                    }
                    ACCOUNT.save(storage, &accounts).unwrap();
                    MARGIN_OPENED_POSITION.save(storage, &vec![]).unwrap();
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
                    MARGIN_OPENED_POSITION.save(storage, &vec![]).unwrap();
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
