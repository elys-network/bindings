use std::str::FromStr;

use crate::msg::query_resp::UserValueResponse;
use crate::msg::SudoMsg;
use crate::msg::{InstantiateMsg, QueryMsg};
use cosmwasm_std::{coins, Addr, BlockInfo, Coin, Decimal, Decimal256, Timestamp};
use cw_multi_test::{BankSudo, BasicAppBuilder, ContractWrapper, Executor, SudoMsg as AppSudo};
use elys_bindings::types::{OracleAssetInfo, Price, SwapAmountInRoute};
use elys_bindings_test::{
    ACCOUNT, ASSET_INFO, LAST_MODULE_USED, PERPETUAL_OPENED_POSITION, PRICES,
};
use trade_shield_contract::entry_point::{
    execute as trade_shield_execute, instantiate as trade_shield_init,
    migrate as trade_shield_migrate, query as trade_shield_query,
};
use trade_shield_contract::msg as trade_shield_msg;
use trade_shield_contract::types::{OrderPrice, SpotOrderType};

use crate::entry_point::instantiate;
use crate::entry_point::{execute, query, sudo};
use anyhow::{bail, Error, Result as AnyResult};
use cosmwasm_std::{to_json_binary, DecCoin, Empty, Int128, SignedDecimal, StdError, Uint128};
use cw_multi_test::{AppResponse, Module};

use elys_bindings::query_resp::{
    AmmSwapEstimationByDenomResponse, DelegationDelegatorReward, Entry, EstakingRewardsResponse,
    MasterchefUserPendingRewardData, MasterchefUserPendingRewardResponse, OracleAssetInfoResponse,
    QueryAllProgramRewardsResponse, QueryGetEntryResponse, QueryGetPriceResponse,
    QueryStableStakeAprResponse, Validator,
};
use elys_bindings::types::BalanceAvailable;
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::ElysModule;

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
            ElysQuery::AmmSwapEstimationByDenom { denom_out, .. } => {
                let resp = AmmSwapEstimationByDenomResponse {
                    in_route: Some(vec![SwapAmountInRoute {
                        pool_id: 1,
                        token_out_denom: denom_out,
                    }]),
                    out_route: None,
                    spot_price: Decimal::from_str("3.5").unwrap(),
                    amount: Coin {
                        denom:
                            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                                .to_string(),
                        amount: Uint128::new(100),
                    },
                    swap_fee: SignedDecimal::one(),
                    discount: SignedDecimal::from_str("20").unwrap(),
                    available_liquidity: Coin {
                        denom: "uelys".to_string(),
                        amount: Uint128::new(100000),
                    },
                    weight_balance_ratio: SignedDecimal::one(),
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
fn history() {
    let wallet: Vec<(&str, Vec<Coin>)> = vec![("user-a", coins(300, "uelys"))];

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
        .unwrap()
        .to_string();

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query).with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMsg {
        limit: Some(3),
        expiration: Some(cw_utils::Expiration::AtTime(Timestamp::from_seconds(
            604800,
        ))),
        trade_shield_address: Some(trade_shield_address.clone()),
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
        Addr::unchecked(trade_shield_address.clone()),
        &trade_shield_msg::MigrateMsg {
            account_history_address: Some(addr.to_string()),
        },
        trade_shield_code_id,
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("user-a"),
        Addr::unchecked(trade_shield_address.clone()),
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

    let update_msg = SudoMsg::ClockEndBlock {};

    // t0
    app.set_block(BlockInfo {
        height: 1,
        time: Timestamp::from_seconds(0 * 24 * 60 * 60),
        chain_id: "elys-app".to_string(),
    });

    app.wasm_sudo(addr.clone(), &update_msg).unwrap();

    let query_msg = QueryMsg::UserValue {
        user_address: "user-a".to_string(),
    };

    let res: UserValueResponse = app.wrap().query_wasm_smart(&addr, &query_msg).unwrap();

    assert_eq!(
        res.value.total_balance_usd,
        Decimal256::from_str("400.0010347342").unwrap(),
    );

    app.sudo(AppSudo::Bank(BankSudo::Mint {
        to_address: "user-a".to_string(),
        amount: coins(200, "uelys"),
    }))
    .unwrap();

    // t1
    app.set_block(BlockInfo {
        height: 2,
        time: Timestamp::from_seconds(1 * 24 * 60 * 60),
        chain_id: "elys-app".to_string(),
    });

    app.wasm_sudo(addr.clone(), &update_msg).unwrap();

    let res: UserValueResponse = app.wrap().query_wasm_smart(&addr, &query_msg).unwrap();

    assert_eq!(
        res.value.total_balance_usd,
        Decimal256::from_str("400.0010347342").unwrap(),
    ); // The previous value wasn't removed yet but wasn't read either since it's expired.
}
