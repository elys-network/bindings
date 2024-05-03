use std::str::FromStr;

use crate::entry_point::{execute, query, sudo};
use crate::tests::test_order_status::test_spot_order_status;
use anyhow::{bail, Result as AnyResult};
use cosmwasm_std::{
    coin, to_json_binary, Addr, BankMsg, BlockInfo, Decimal, Empty, Int64, SignedDecimal, StdError,
    Timestamp,
};
use cw_multi_test::BankSudo;
use cw_multi_test::{AppResponse, BasicAppBuilder, ContractWrapper, Executor, Module};
use elys_bindings::msg_resp::AmmSwapExactAmountInResp;
use elys_bindings::query_resp::{
    AmmSwapEstimationByDenomResponse, Entry, QueryGetEntryResponse, QueryGetPriceResponse,
};

use elys_bindings::trade_shield::msg::SudoMsg;
use elys_bindings::trade_shield::types::{OrderPrice, SpotOrder, SpotOrderType, Status};
use elys_bindings::types::{Price, SwapAmountInRoute, SwapAmountOutRoute};
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::{
    ElysModule, ACCOUNT, ASSET_INFO, LAST_MODULE_USED, PERPETUAL_OPENED_POSITION, PRICES,
};

use super::InstantiateMockMsg;
use super::{instantiate, reply};

struct ElysModuleWrapper(ElysModule);

impl Module for ElysModuleWrapper {
    type QueryT = ElysQuery;
    type ExecT = ElysMsg;
    type SudoT = Empty;

    fn query(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &dyn cosmwasm_std::Storage,
        _querier: &dyn cosmwasm_std::Querier,
        _block: &cosmwasm_std::BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        match request {
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
            ElysQuery::AmmSwapEstimationByDenom {
                amount,
                denom_in,
                denom_out,
                ..
            } => {
                let spot_price = match (denom_in.as_str(), denom_out.as_str()) {
                    (
                        "uelys",
                        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
                    ) => Decimal::from_str("3.88").unwrap(),
                    _ => panic!(
                        "price not found for the pair of {} and {}",
                        denom_in, denom_out
                    ),
                };
                let amount_value = amount.amount * spot_price.clone();

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
                            token_in_denom: denom_in.clone(),
                        }]),
                    )
                };

                let resp = AmmSwapEstimationByDenomResponse {
                    in_route: in_route,
                    out_route: out_route,
                    spot_price,
                    amount: coin(amount_value.u128(), &denom_in),
                    swap_fee: SignedDecimal::zero(),
                    discount: SignedDecimal::zero(),
                    available_liquidity: coin(95841644452, &denom_in),
                    weight_balance_ratio: SignedDecimal::zero(),
                    price_impact: SignedDecimal::zero(),
                    slippage: Decimal::zero(),
                };

                Ok(to_json_binary(&resp)?)
            }

            ElysQuery::OracleAssetInfo { .. } => {
                Err(StdError::generic_err("not_implemented").into())
            }
            ElysQuery::AmmPriceByDenom { token_in, .. } => {
                let spot_price = match token_in.denom.as_str() {
                    "uelys" => Decimal::from_str("3.88").unwrap(),
                    _ => panic!("price not found for {}", token_in.denom),
                };

                Ok(to_json_binary(&spot_price)?)
            }
            ElysQuery::OraclePrice { asset, .. } => match asset.as_str() {
                "USDC" => Ok(to_json_binary(&QueryGetPriceResponse {
                    price: Price {
                        asset,
                        price: Decimal::one(),
                        source: "".to_string(),
                        provider: "".to_string(),
                        timestamp: 0,
                        block_height: 0,
                    },
                })?),
                _ => panic!("price not found for {}", asset),
            },

            _ => panic!("not implemented"),
        }
    }

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        _sender: Addr,
        msg: Self::ExecT,
    ) -> AnyResult<AppResponse>
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
                recipient,
                token_out_min_amount,
                ..
            } => {
                let burn_msg = BankMsg::Burn {
                    amount: vec![token_in.clone()],
                };
                router
                    .execute(
                        api,
                        storage,
                        block,
                        Addr::unchecked(sender.clone()),
                        burn_msg.into(),
                    )
                    .unwrap();

                let price = match (token_in.denom.as_str(), routes[0].token_out_denom.as_str()) {
                    (
                        "uelys",
                        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
                    ) => Decimal::from_str("3.88").unwrap(),
                    _ => panic!(
                        "price not found for the pair of {} and {}",
                        token_in.denom, routes[0].token_out_denom
                    ),
                };

                let mint_amount: u128 = price
                    .checked_mul(Decimal::from_atomics(token_in.amount, 0).unwrap())
                    .unwrap()
                    .checked_mul(Decimal::from_str("0.99").unwrap())
                    .unwrap()
                    .to_uint_floor()
                    .u128();

                if mint_amount < token_out_min_amount.i128() as u128 {
                    panic!("insufficient amount to mint");
                }

                let mint_msg = BankSudo::Mint {
                    to_address: recipient.clone(),
                    amount: vec![coin(mint_amount, &routes[0].token_out_denom)],
                };
                router.sudo(api, storage, block, mint_msg.into()).unwrap();

                let data = to_json_binary(&AmmSwapExactAmountInResp {
                    token_out_amount: Int64::new(mint_amount as i64),
                    discount: Decimal::zero(),
                    swap_fee: Decimal::from_str("0.1").unwrap(),
                    recipient,
                })?;

                let resp = AppResponse {
                    events: vec![],
                    data: Some(data),
                };
                Ok(resp)
            }
            _ => panic!("not implemented"),
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        bail!("sudo is not implemented for ElysModule")
    }
}

#[test]
fn process_limit_buy_order_with_executed_status() {
    // Create a wallet for the "user" with an initial balance of 100 usdc
    let wallet = vec![
        ("user", vec![coin(90_000000, "uelys")]),
        ("owner", vec![coin(10_000000, "uelys")]),
    ];

    let mut addresses: Vec<String> = vec![];
    let mut app = BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
        .with_custom(ElysModuleWrapper(ElysModule {}))
        .build(|roouter, _, storage| {
            for (wallet_owner, wallet_contenent) in wallet {
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
        });

    // trade shield deployment
    let code = ContractWrapper::new(execute, instantiate, query)
        .with_sudo(sudo)
        .with_reply(reply);
    let code_id = app.store_code(Box::new(code));

    // Create a "limit buy" order with a specific rate and balance.
    let order = SpotOrder::new(
        0,
        SpotOrderType::LimitBuy,
        Some(OrderPrice {
            // denom_in
            base_denom: "uelys".to_string(),
            // denom_out
            quote_denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                .to_string(),
            rate: Decimal::from_str("0.5").unwrap(),
        }),
        coin(10_000000, "uelys"),
        Addr::unchecked("user"),
        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
        &BlockInfo {
            height: 50,
            time: Timestamp::from_seconds(600),
            chain_id: "elys-app".to_string(),
        },
    );

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        account_history_address: None,
        spot_orders: vec![order.clone()],
        perpetual_orders: vec![],
    };

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &[coin(10_000000, "uelys")],
            "Contract",
            None,
        )
        .unwrap();

    test_spot_order_status(
        &app.wrap(),
        addr.to_string(),
        order.order_id,
        Status::Pending,
    );

    app.wasm_sudo(addr.clone(), &SudoMsg::ClockEndBlock {})
        .unwrap();

    test_spot_order_status(
        &app.wrap(),
        addr.to_string(),
        order.order_id,
        Status::Executed,
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "uelys")
            .unwrap()
            .amount
            .u128(),
        0
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "uelys")
            .unwrap()
            .amount
            .u128(),
        90_000000
    );

    assert_eq!(
        app.wrap()
            .query_balance(
                "user",
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            )
            .unwrap()
            .amount
            .u128(),
        38412000
    );
}
