use std::str::FromStr;

use crate::entry_point::{execute, query, sudo};
use crate::tests::get_order_id_from_events::get_order_id_from_events;
use anyhow::{bail, Error, Result as AnyResult};
use cosmwasm_std::{
    coin, coins, to_json_binary, Addr, BankMsg, BlockInfo, Decimal, Empty, SignedDecimal, StdError,
    Timestamp, Uint128,
};
use cw_multi_test::{AppResponse, BasicAppBuilder, ContractWrapper, Executor, Module};
use elys_bindings::query_resp::{
    AmmSwapEstimationByDenomResponse, Entry, OracleAssetInfoResponse, QueryGetEntryResponse,
    QueryGetPriceResponse,
};
use elys_bindings::trade_shield::msg::{ExecuteMsg, SudoMsg};
use elys_bindings::trade_shield::types::{OrderPrice, SpotOrder, SpotOrderType};
use elys_bindings::types::{OracleAssetInfo, Price, SwapAmountInRoute, SwapAmountOutRoute};
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::{
    ElysModule, ACCOUNT, ASSET_INFO, LAST_MODULE_USED, PERPETUAL_OPENED_POSITION, PRICES,
};

use super::instantiate;
use super::InstantiateMockMsg;

struct ElysModuleWrapper(ElysModule);

impl Module for ElysModuleWrapper {
    type QueryT = ElysQuery;
    type ExecT = ElysMsg;
    type SudoT = Empty;

    fn query(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &dyn cosmwasm_std::Storage,
        querier: &dyn cosmwasm_std::Querier,
        block: &cosmwasm_std::BlockInfo,
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
                    _ => return Err(Error::new(StdError::not_found(base_denom))),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::AmmPriceByDenom { token_in, .. } => {
                let spot_price = match token_in.denom.as_str() {
                    "uelys" => Decimal::from_str("0.283221851948960688").unwrap(),
                    _ => return Err(Error::new(StdError::not_found(token_in.denom.as_str()))),
                };
                Ok(to_json_binary(&spot_price)?)
            }
            ElysQuery::OraclePrice { asset, .. } => {
                let resp = match asset.as_str() {
                    "USDC" => QueryGetPriceResponse {
                        price: Price {
                            asset: "USDC".to_string(),
                            price: Decimal::one(),
                            source: "uelys".to_string(),
                            provider: "elys1wzm8dvpxpxxf26y4xn85w5adakcenprg4cq2uf".to_string(),
                            // set timestamp to now
                            timestamp: block.time.seconds(),
                        },
                    },
                    _ => return Err(Error::new(StdError::not_found(asset))),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::OracleAssetInfo { denom } => {
                let resp = match denom.as_str() {
                    "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65" => {
                        OracleAssetInfoResponse {
                            asset_info: OracleAssetInfo {
                                band_ticker: "USDC".to_string(),
                                decimal: 6,
                                denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
                                display: "USDC".to_string(),
                                elys_ticker: "USDC".to_string(),
                            },
                        }
                    }
                    "ibc/47BD209179859CDE4A2806763D7189B6E6FE13A17880FE2B42DE1E6C1E329E23" => {
                        OracleAssetInfoResponse {
                            asset_info: OracleAssetInfo {
                                band_ticker: "OSMO".to_string(),
                                decimal: 6,
                                denom: "ibc/47BD209179859CDE4A2806763D7189B6E6FE13A17880FE2B42DE1E6C1E329E23".to_string(),
                                display: "OSMO".to_string(),
                                elys_ticker: "OSMO".to_string(),
                            },
                        }
                    }
                    "ibc/977D5388D2FBE72D9A33FE2423BF8F4DADF3B591207CC98A295B9ACF81E4DE40" => {
                        OracleAssetInfoResponse {
                            asset_info: OracleAssetInfo {
                                band_ticker: "JUNO".to_string(),
                                decimal: 6,
                                denom: "ibc/977D5388D2FBE72D9A33FE2423BF8F4DADF3B591207CC98A295B9ACF81E4DE40".to_string(),
                                display: "JUNO".to_string(),
                                elys_ticker: "JUNO".to_string(),
                            },
                        }
                    }
                    "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4" => {
                        OracleAssetInfoResponse {
                            asset_info: OracleAssetInfo {
                                band_ticker: "ATOM".to_string(),
                                decimal: 6,
                                denom: "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4".to_string(),
                                display: "ATOM".to_string(),
                                elys_ticker: "ATOM".to_string(),
                            },
                        }
                    },
                    _ => return Err(Error::new(StdError::not_found(denom))),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::AmmSwapEstimationByDenom {
                amount,
                denom_in,
                denom_out,
                ..
            } => {
                let spot_price = match denom_in.as_str() {
                    "uelys" => Decimal::from_str("3.5308010067676894").unwrap(),
                    "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65" => {
                        Decimal::one()
                    }
                    "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4" => {
                        Decimal::from_str("9.02450744362719844").unwrap()
                    }
                    _ => return Err(Error::new(StdError::not_found(denom_in.as_str()))),
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
                };

                Ok(to_json_binary(&resp)?)
            }
            _ => self.0.query(api, storage, querier, block, request),
        }
    }

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        sender: Addr,
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
                token_out_min_amount,
                discount,
                recipient,
            } => {
                let resp = AppResponse {
                    events: vec![],
                    data: Some(to_json_binary(&token_in)?),
                };
                let burn_msg = BankMsg::Burn {
                    amount: vec![token_in],
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
                Ok(resp)
            }
            _ => self.0.execute(api, storage, router, block, sender, msg),
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
fn process_limit_buy_order_with_testnet_data() {
    // Create a wallet for the "user" with an initial balance of 100 usdc
    let wallet = vec![(
        "user",
        vec![
            coin(
                100_000000,
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
            ),
            coin(100_000000, "uelys"),
        ],
    )];

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
    let code = ContractWrapper::new(execute, instantiate, query).with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    // Create a "limit buy" order with a specific rate and balance.
    let order = SpotOrder::new(
        0,
        SpotOrderType::LimitBuy,
        Some(OrderPrice {
            base_denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                .to_string(),
            quote_denom: "uelys".to_string(),
            rate: Decimal::from_atomics(Uint128::new(1), 0).unwrap(),
        }),
        coin(
            10_000000,
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
        ),
        Addr::unchecked("user"),
        "uelys".to_string(),
        &BlockInfo {
            height: 50,
            time: Timestamp::from_seconds(600),
            chain_id: "elys-app".to_string(),
        },
    );

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![order],
        perpetual_orders: vec![],
    };

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();

    // let resp = app
    //     .execute_contract(
    //         Addr::unchecked("user"),
    //         addr.clone(),
    //         &ExecuteMsg::CreateSpotOrder {
    //             order_type: SpotOrderType::LimitBuy,
    //             order_price: Some(OrderPrice {
    //                 base_denom:
    //                     "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
    //                         .to_string(),
    //                 quote_denom: "uelys".to_string(),
    //                 rate: Decimal::from_atomics(Uint128::new(30000), 0).unwrap(), // The maximum price of 30000 USDC per BTC.
    //             }),

    //             order_source_denom:
    //                 "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
    //                     .to_string(),
    //             order_target_denom: "uelys".to_string(),
    //         },
    //         &coins(
    //             10_000000,
    //             "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
    //         ),
    //     )
    //     .unwrap();

    // // Verify that the "user" has less USDC left after creating the order.
    // assert_eq!(
    //     app.wrap()
    //         .query_balance("user", "usdc")
    //         .unwrap()
    //         .amount
    //         .u128(),
    //     90_000000
    // );

    // // Verify that the contract address now holds the 10 USDC for the order.
    // assert_eq!(
    //     app.wrap()
    //         .query_balance(&addr, "usdc")
    //         .unwrap()
    //         .amount
    //         .u128(),
    //     10_000000
    // );

    // // Verify that an order ID is emitted in the contract's events.
    // assert!(get_order_id_from_events(&resp.events).is_some());

    app.wasm_sudo(addr.clone(), &SudoMsg::ClockEndBlock {})
        .unwrap();
}
