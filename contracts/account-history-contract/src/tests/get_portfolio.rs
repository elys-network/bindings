use std::str::FromStr;

use crate::entry_point::instantiate;
use crate::msg::query_resp::GetPortfolioResp;
use crate::{
    entry_point::{execute, query, sudo},
    msg::*,
};
use anyhow::{bail, Error, Result as AnyResult};
use cosmwasm_std::{
    coin, coins, to_json_binary, Addr, BlockInfo, Decimal, Decimal256, Empty, SignedDecimal, Int128,
    SignedDecimal256, StdError, Timestamp, Uint128,
};
use cw_multi_test::{AppResponse, BankSudo, BasicAppBuilder, ContractWrapper, Executor, Module};
use cw_utils::Expiration;
use elys_bindings::account_history::msg::query_resp::StakeAssetBalanceBreakdown;
use elys_bindings::account_history::types::{Portfolio, PortfolioBalanceSnapshot};
use elys_bindings::query_resp::{
    AmmSwapEstimationByDenomResponse, DelegationDelegatorReward, Entry, EstakingRewardsResponse,
    MasterchefUserPendingRewardData, MasterchefUserPendingRewardResponse, OracleAssetInfoResponse,
    QueryAllProgramRewardsResponse, QueryGetEntryResponse, QueryGetPriceResponse,
    QueryStableStakeAprResponse, Validator,
};
use elys_bindings::types::{BalanceAvailable, OracleAssetInfo, Price, SwapAmountInRoute};
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::{
    ElysModule, ACCOUNT, ASSET_INFO, LAST_MODULE_USED, PERPETUAL_OPENED_POSITION, PRICES,
};
use trade_shield_contract::entry_point::{
    execute as trade_shield_execute, instantiate as trade_shield_init,
    migrate as trade_shield_migrate, query as trade_shield_query,
};
use trade_shield_contract::msg as trade_shield_msg;
use trade_shield_contract::types::{OrderPrice, SpotOrderType};
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
            ElysQuery::AmmBalance { .. } => {
                let resp = BalanceAvailable {
                    amount: Uint128::new(0),
                    usd_amount: Decimal::zero(),
                };
                Ok(to_json_binary(&resp)?)
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
                    _ => return Err(Error::new(StdError::not_found(base_denom))),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::AmmPriceByDenom { token_in, .. } => {
                let spot_price = match token_in.denom.as_str() {
                    "uelys" => Decimal::from_str("3.449114").unwrap(),
                    "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65" => {
                        Decimal::one()
                    }
                    "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4" => {
                        Decimal::from_str("9.165195").unwrap()
                    }
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
                            block_height: block.height,
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
                denom_out,
                discount,
                ..
            } => {
                let resp = AmmSwapEstimationByDenomResponse {
                    in_route: Some(vec![SwapAmountInRoute {
                        pool_id: 1,
                        token_out_denom: denom_out.clone(),
                    }]),
                    out_route: None,
                    spot_price: Decimal::one(),
                    amount,
                    swap_fee: SignedDecimal::zero(),
                    discount: SignedDecimal::from_str(discount.to_string().as_str()).unwrap(),
                    available_liquidity: coin(999999999, denom_out),
                    weight_balance_ratio: SignedDecimal::zero(),
                    price_impact: SignedDecimal::zero(),
                    slippage: Decimal::zero(),
                };
                Ok(to_json_binary(&resp)?)

            }
            ElysQuery::EstakingRewards { .. } => {
                let resp = EstakingRewardsResponse {
                    rewards: vec![DelegationDelegatorReward {
                        validator_address: Validator::EdenBoost.to_string(),
                        reward: vec![DecCoin {
                            denom: "ueden".to_string(),
                            amount: Decimal256::from_str("1.21").unwrap(),
                        }],
                    }],
                    total: vec![DecCoin {
                        denom: "uedenb".to_string(),
                        amount: Decimal256::from_str("1.21").unwrap(),
                    }],
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::MasterchefStableStakeApr { .. } => {
                let resp = QueryStableStakeAprResponse {
                    apr: Int128::new(12),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::MasterchefUserPendingReward { .. } => {
                let resp = MasterchefUserPendingRewardResponse {
                    rewards: vec![MasterchefUserPendingRewardData {
                        pool_id: 32767u64,
                        reward: vec![Coin {
                            denom: "ueden".to_string(),
                            amount: Uint128::new(20),
                        }],
                    }],
                    total_rewards: vec![Coin {
                        denom: "ueden".to_string(),
                        amount: Uint128::new(20),
                    }],
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::IncentiveAllProgramRewards { .. } => {
                let resp = QueryAllProgramRewardsResponse {
                    usdc_staking_rewards: vec![DecCoin {
                        denom:
                            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                                .to_string(),
                        amount: Decimal256::from_str("10").unwrap(),
                    }],
                    elys_staking_rewards: vec![DecCoin {
                        denom: "uelys".to_string(),
                        amount: Decimal256::from_str("10").unwrap(),
                    }],
                    eden_staking_rewards: vec![DecCoin {
                        denom: "ueden".to_string(),
                        amount: Decimal256::from_str("10").unwrap(),
                    }],
                    edenb_staking_rewards: vec![DecCoin {
                        denom: "uedenb".to_string(),
                        amount: Decimal256::from_str("10").unwrap(),
                    }],
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
fn get_portfolio() {
    // Create a wallet for the "user" with an initial balance of 100 usdc
    let wallet = vec![(
        "user",
        vec![
            coin(
                1445910542,
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
            ),
            coin(
                19295155,
                "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4",
            ),
            coin(104332087, "uelys"),
        ],
    )];

    let mut addresses: Vec<String> = vec![];
    let mut app = BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
        .with_custom(ElysModuleWrapper(ElysModule {}))
        .build(|router, _, storage| {
            for (wallet_owner, wallet_contenent) in wallet {
                router
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
    let trade_shield_code =
        ContractWrapper::new(trade_shield_execute, trade_shield_init, trade_shield_query)
            .with_migrate(trade_shield_migrate);
    let trade_shield_code_id = app.store_code(Box::new(trade_shield_code));
    let trade_shield_init = trade_shield_msg::InstantiateMsg {
        account_history_address: None,
    };

    let trade_shield_address = app
        .instantiate_contract(
            trade_shield_code_id,
            Addr::unchecked("owner"),
            &trade_shield_init,
            &[],
            "Contract",
            Some("admin".to_string()),
        )
        .unwrap();

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query).with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMsg {
        limit: Some(3),
        expiration: Some(cw_utils::Expiration::AtTime(Timestamp::from_seconds(
            604800,
        ))),
        trade_shield_address: Some(trade_shield_address.to_string()),
    };

    // Instantiate the contract with "owner" as the deployer.
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

    app.migrate_contract(
        Addr::unchecked("admin"),
        trade_shield_address.clone(),
        &trade_shield_msg::MigrateMsg {
            account_history_address: Some(addr.to_string()),
        },
        trade_shield_code_id,
    )
    .unwrap();

    // t0
    app.set_block(BlockInfo {
        height: 1,
        time: Timestamp::from_seconds(0),
        chain_id: "elys".to_string(),
    });

    app.execute_contract(
        Addr::unchecked("user"),
        trade_shield_address.clone(),
        &trade_shield_msg::ExecuteMsg::CreateSpotOrder {
            order_type: SpotOrderType::StopLoss,
            order_source_denom: "uelys".to_string(),
            order_target_denom:
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
            order_price: Some(OrderPrice {
                base_denom: "uelys".to_string(),
                quote_denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                    .to_string(),
                rate: Decimal::one(),
            }),
        },
        &coins(100, "uelys"),
    )
    .unwrap();

    // update account
    app.wasm_sudo(addr.clone(), &SudoMsg::ClockEndBlock {})
        .unwrap();

    // Query the contract for the existing order.
    let resp: GetPortfolioResp = app
        .wrap()
        .query_wasm_smart(
            &addr,
            &QueryMsg::GetPortfolio {
                user_address: "user".to_string(),
            },
        )
        .unwrap();

    let expected = GetPortfolioResp {
        actual_portfolio_balance: SignedDecimal256::from_str("2382.607662051143").unwrap(),
        portfolio: Portfolio {
            balance_usd: Decimal256::from_str("2382.607662051143").unwrap(),

            liquid_assets_usd: Decimal256::from_str("1982.607662051143").unwrap(),

            staked_committed_usd: Decimal256::from_str("400").unwrap(),

            liquidity_positions_usd: Decimal256::from_str("0").unwrap(),

            leverage_lp_usd: Decimal256::from_str("0").unwrap(),

            perpetual_assets_usd: Decimal256::from_str("0").unwrap(),

            usdc_earn_usd: Decimal256::from_str("0").unwrap(),

            borrows_usd: Decimal256::from_str("0").unwrap(),
            stake_balance_breakdown: StakeAssetBalanceBreakdown {
                staked: Decimal::from_str("300").unwrap_or_default(),
                unstaking: Decimal::from_str("0").unwrap_or_default(),
                vesting: Decimal::from_str("100").unwrap_or_default(),
            },
        },
    };

    // test if the response is the same as the expected
    assert_eq!(resp.portfolio.balance_usd, expected.portfolio.balance_usd);
    assert_eq!(
        resp.portfolio.liquid_assets_usd,
        expected.portfolio.liquid_assets_usd
    );
    assert_eq!(
        resp.portfolio.staked_committed_usd,
        expected.portfolio.staked_committed_usd
    );
    assert_eq!(
        resp.portfolio.liquidity_positions_usd,
        expected.portfolio.liquidity_positions_usd
    );
    assert_eq!(
        resp.portfolio.leverage_lp_usd,
        expected.portfolio.leverage_lp_usd
    );
    assert_eq!(
        resp.portfolio.perpetual_assets_usd,
        expected.portfolio.perpetual_assets_usd
    );
    assert_eq!(
        resp.portfolio.usdc_earn_usd,
        expected.portfolio.usdc_earn_usd
    );
    assert_eq!(resp.portfolio.borrows_usd, expected.portfolio.borrows_usd);
    assert_eq!(resp, expected);

    // t1 (1d later)
    app.set_block(BlockInfo {
        height: 2,
        time: Timestamp::from_seconds(24 * 60 * 60),
        chain_id: "elys".to_string(),
    });

    // mint some coins
    app.sudo(
        BankSudo::Mint {
            to_address: "user".to_string(),
            amount: vec![coin(100000000, "uelys")],
        }
        .into(),
    )
    .unwrap();

    // update account
    app.wasm_sudo(addr.clone(), &SudoMsg::ClockEndBlock {})
        .unwrap();

    // t2 (2d later)
    app.set_block(BlockInfo {
        height: 3,
        time: Timestamp::from_seconds(24 * 60 * 60 * 2),
        chain_id: "elys".to_string(),
    });

    // mint some coins
    app.sudo(
        BankSudo::Mint {
            to_address: "user".to_string(),
            amount: vec![coin(300000000, "uelys")],
        }
        .into(),
    )
    .unwrap();

    // update account
    app.wasm_sudo(addr.clone(), &SudoMsg::ClockEndBlock {})
        .unwrap();

    // t3 (3d later)
    app.set_block(BlockInfo {
        height: 4,
        time: Timestamp::from_seconds(24 * 60 * 60 * 3),
        chain_id: "elys".to_string(),
    });

    // mint some coins
    app.sudo(
        BankSudo::Mint {
            to_address: "user".to_string(),
            amount: vec![coin(50000000, "uelys")],
        }
        .into(),
    )
    .unwrap();

    // update account
    app.wasm_sudo(addr.clone(), &SudoMsg::ClockEndBlock {})
        .unwrap();

    // Query the contract for the existing order.
    let last_snapshot: PortfolioBalanceSnapshot = app
        .wrap()
        .query_wasm_smart(
            &addr,
            &QueryMsg::LastSnapshot {
                user_address: "user".to_string(),
            },
        )
        .unwrap();

    // test if the response is the same as the expected
    assert_eq!(
        last_snapshot.date,
        Expiration::AtTime(Timestamp::from_seconds(24 * 60 * 60 * 3))
    );

    // Query the contract for the existing order.
    let resp: GetPortfolioResp = app
        .wrap()
        .query_wasm_smart(
            &addr,
            &QueryMsg::GetPortfolio {
                user_address: "user".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        resp.actual_portfolio_balance,
        SignedDecimal256::from_str("3934.708962051143").unwrap()
    );
}
