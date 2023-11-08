use std::ops::{Deref, DerefMut};

use anyhow::{bail, Error, Result as AnyResult};
use cosmwasm_std::{
    coin, coins,
    testing::{MockApi, MockStorage},
    to_json_binary, Addr, BankMsg, BlockInfo, Coin, Decimal, Empty, Int64, Querier, StdError,
    StdResult, Storage,
};
use cw_multi_test::{App, AppResponse, BankKeeper, BankSudo, BasicAppBuilder, Module, WasmKeeper};
use cw_storage_plus::Item;
use elys_bindings::{
    msg_resp::{MsgCloseResponse, MsgOpenResponse, MsgSwapExactAmountInResp},
    query_resp::QuerySwapEstimationResponse,
    types::{AssetInfo, MarginOrder, MarginPosition, Price},
    AmmMsg, AmmQuery, ElysMsg, ElysQuery, MarginMsg, OracleQuery,
};
use std::cmp::max;

pub const PRICES: Item<Vec<Price>> = Item::new("prices");
pub const ASSET_INFO: Item<Vec<AssetInfo>> = Item::new("asset_info");
pub const BLOCK_TIME: u64 = 5;
pub const MARGIN_OPENED_POSITION: Item<Vec<MarginOrder>> = Item::new("margin_opened_position");

pub struct ElysModule {}

impl ElysModule {
    fn get_all_price(&self, store: &dyn Storage) -> StdResult<Vec<Price>> {
        Ok(PRICES.load(store)?)
    }

    pub fn set_prices(&self, store: &mut dyn Storage, prices: &Vec<Price>) -> StdResult<()> {
        PRICES.save(store, prices)
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
        infos: &Vec<AssetInfo>,
    ) -> StdResult<()> {
        ASSET_INFO.save(store, infos)
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
            ElysQuery::Oracle(oracle_req) => match oracle_req {
                OracleQuery::PriceAll { .. } => Ok(to_json_binary(&self.get_all_price(storage)?)?),
                OracleQuery::AssetInfo { denom } => {
                    let infos = ASSET_INFO.load(storage)?;
                    let may_have_info = infos.iter().find(|asset| asset.denom == denom);

                    match may_have_info {
                        Some(info) => Ok(to_json_binary(info)?),
                        None => Err(Error::new(StdError::not_found("asset denom"))),
                    }
                }
            },
            ElysQuery::Amm(amm_req) => match amm_req {
                AmmQuery::QuerySwapEstimation { routes, token_in } => {
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

                    Ok(to_json_binary(&QuerySwapEstimationResponse {
                        spot_price,
                        token_out: coin(token_out_amount, &routes[0].token_out_denom),
                    })?)
                }
            },
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
            ElysMsg::Amm(amm_msg) => match amm_msg {
                AmmMsg::MsgSwapExactAmountIn {
                    sender,
                    routes,
                    token_in,
                    token_out_min_amount,
                    meta_data,
                } => {
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

                    if mint_amount[0].amount.u128() as i128 <= token_out_min_amount.i128() {
                        return Err(Error::new(StdError::generic_err("not enough token")));
                    }

                    let data = to_json_binary(&MsgSwapExactAmountInResp {
                        token_out_amount: Int64::new(mint_amount[0].amount.u128() as i64),

                        meta_data,
                    })?;

                    let mint = BankSudo::Mint {
                        to_address: sender.clone(),
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
            },
            ElysMsg::Margin(margin_msg) => match margin_msg {
                MarginMsg::MsgOpen {
                    creator,
                    collateral_asset,
                    collateral_amount,
                    borrow_asset,
                    position,
                    leverage,
                    take_profit_price,
                    meta_data,
                } => {
                    let mut order_vec = MARGIN_OPENED_POSITION.load(storage)?;

                    let order_id: u64 = match order_vec.iter().max_by_key(|s| s.order_id) {
                        Some(x) => x.order_id + 1,
                        None => 0,
                    };
                    let collateral = coin(collateral_amount.i128() as u128, collateral_asset);

                    let borrow_token = Coin {
                        denom: borrow_asset,
                        amount: leverage * collateral.amount,
                    };

                    let prices = PRICES.load(storage)?;
                    let price = prices
                        .iter()
                        .find(|price| price.asset == collateral.denom)
                        .unwrap();

                    let order: MarginOrder = MarginOrder {
                        order_id,
                        position: MarginPosition::try_from_i32(position).unwrap(),
                        collateral: collateral.clone(),
                        borrow_token,
                        creator,
                        leverage,
                        take_profit_price,
                        token_price: price.price,
                    };

                    let msg_resp = MsgOpenResponse {
                        id: order_id,
                        meta_data,
                    };

                    let resp = AppResponse {
                        events: vec![],
                        data: Some(to_json_binary(&msg_resp)?),
                    };

                    order_vec.push(order);

                    let burn_msg = BankMsg::Burn {
                        amount: vec![collateral],
                    };
                    router
                        .execute(api, storage, block, sender, burn_msg.into())
                        .unwrap();

                    Ok(resp)
                }

                MarginMsg::MsgClose {
                    creator,
                    id,
                    meta_data,
                } => {
                    let orders = MARGIN_OPENED_POSITION.load(storage)?;
                    let prices = PRICES.load(storage)?;

                    let order = match orders.iter().find(|order| order.order_id == id) {
                        Some(order) => order.clone(),
                        None => return Err(Error::new(StdError::not_found(format!("{id}")))),
                    };

                    let price = prices
                        .iter()
                        .find(|price| price.asset == order.collateral.denom)
                        .unwrap();

                    if order.creator != sender || order.creator != creator {
                        return Err(Error::new(StdError::generic_err(
                            "Unauthtorized".to_string(),
                        )));
                    }

                    let new_orders_list: Vec<MarginOrder> = orders
                        .into_iter()
                        .filter(|order| order.order_id != id)
                        .collect();

                    MARGIN_OPENED_POSITION.save(storage, &new_orders_list)?;

                    let price_variation = match order.position {
                        MarginPosition::Long => price.price / order.token_price,
                        MarginPosition::Short => order.token_price / price.price,
                        MarginPosition::Unspecified => Decimal::one(),
                    };

                    let mint_amount = (price_variation * order.leverage * order.collateral.amount)
                        - order.borrow_token.amount;

                    let mint_msg: Option<BankSudo> = if mint_amount.u128() > 0 {
                        Some(BankSudo::Mint {
                            to_address: creator,
                            amount: vec![coin(mint_amount.u128(), order.collateral.denom)],
                        })
                    } else {
                        None
                    };

                    if let Some(mint_msg) = mint_msg {
                        router.sudo(api, storage, block, mint_msg.into())?;
                    };

                    let data = Some(to_json_binary(&MsgCloseResponse { id, meta_data })?);

                    Ok(AppResponse {
                        events: vec![],
                        data,
                    })
                }
            },
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
        Self(
            BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
                .with_custom(ElysModule {})
                .build(|roouter, _, storage| {
                    for (wallet_owner, wallet_contenent) in wallets {
                        roouter
                            .bank
                            .init_balance(storage, &Addr::unchecked(wallet_owner), wallet_contenent)
                            .unwrap();
                    }
                    MARGIN_OPENED_POSITION.save(storage, &vec![]).unwrap();
                    ASSET_INFO.save(storage, &vec![]).unwrap();
                    PRICES.save(storage, &vec![]).unwrap();
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
