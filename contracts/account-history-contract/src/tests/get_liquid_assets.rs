use std::str::FromStr;

use crate::states::{EXPIRATION, PAGINATION, TRADE_SHIELD_ADDRESS};
use crate::tests::get_liquid_assets::query_resp::{GetLiquidAssetsResp, LiquidAsset};
use crate::{
    entry_point::{execute, query, sudo},
    msg::*,
};
use anyhow::{bail, Error, Result as AnyResult};
use cosmwasm_std::{
    coin, to_json_binary, Addr, DecCoin, Decimal, Decimal256, DepsMut, Empty, Env, MessageInfo,
    Response, StdError, StdResult, Timestamp,
};
use cw_multi_test::{AppResponse, BasicAppBuilder, ContractWrapper, Executor, Module};
use elys_bindings::query_resp::{
    Entry, OracleAssetInfoResponse, QueryGetEntryResponse, QueryGetPriceResponse,
};
use elys_bindings::types::{OracleAssetInfo, PageRequest, Price};
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::{
    ElysModule, ACCOUNT, ASSET_INFO, LAST_MODULE_USED, PERPETUAL_OPENED_POSITION, PRICES,
};
use trade_shield_contract::entry_point::{
    execute as trade_shield_execute, instantiate as trade_shield_init, query as trade_shield_query,
};
use trade_shield_contract::msg::InstantiateMsg as TradeShieldInstantiateMsg;

fn mock_instantiate(
    deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<ElysMsg>> {
    EXPIRATION.save(deps.storage, &msg.expiration)?;
    PAGINATION.save(
        deps.storage,
        &PageRequest {
            key: None,
            limit: msg.limit,
            reverse: false,
            offset: None,
            count_total: false,
        },
    )?;
    TRADE_SHIELD_ADDRESS.save(deps.storage, &msg.trade_shield_address)?;
    Ok(Response::new())
}

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
                    "uelys" => Decimal::from_str("3.5308010067676894").unwrap(),
                    "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65" => {
                        Decimal::one()
                    }
                    "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4" => {
                        Decimal::from_str("9.02450744362719844").unwrap()
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
            ElysQuery::AmmSwapEstimationByDenom { .. } => {
                panic!("not implemented")
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
fn get_liquid_assets() {
    // Create a wallet for the "user" with an initial balance of 100 usdc
    let wallet = vec![(
        "user",
        vec![
            coin(
                21798000,
                "ibc/0E1517E2771CA7C03F2ED3F9BAECCAEADF0BFD79B89679E834933BC0F179AD98",
            ),
            coin(
                5333229342748,
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
            ),
            coin(
                2704998,
                "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
            ),
            coin(
                594000000000200000,
                "ibc/2FBCFC209420E6CECED6EE0BC599E74349759352CE953E27A6871BB3D84BC058",
            ),
            coin(
                1085352,
                "ibc/326A89923D85047E6418A671FBACCAFA2686B01A16ED4A0AD92954FCE1485910",
            ),
            coin(
                168400000000000000,
                "ibc/43881AB3B3D05FD9D3606D7F57CBE6EEEA89D18AC66AF9E2915ED43940E71CFD",
            ),
            coin(
                49765000,
                "ibc/4DAE26570FD24ABA40E2BE4137E39D946C78B00B248D3F78B0919567C4371156",
            ),
            coin(
                9100000,
                "ibc/977D5388D2FBE72D9A33FE2423BF8F4DADF3B591207CC98A295B9ACF81E4DE40",
            ),
            coin(
                141000000000000000,
                "ibc/E059CD828E5009D4CF03C4494BEA73749250287FC98DD46E19F9016B918BF49D",
            ),
            coin(
                37403942,
                "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4",
            ),
            coin(
                79979999999749000,
                "ibc/FB22E35236996F6B0B1C9D407E8A379A7B1F4083F1960907A1622F022AE450E1",
            ),
            coin(45666543, "uelys"),
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
    let trade_shield_code =
        ContractWrapper::new(trade_shield_execute, trade_shield_init, trade_shield_query);
    let trade_shield_code_id = app.store_code(Box::new(trade_shield_code));
    let trade_shield_init = TradeShieldInstantiateMsg {};
    let trade_shield_address = app
        .instantiate_contract(
            trade_shield_code_id,
            Addr::unchecked("owner"),
            &trade_shield_init,
            &[],
            "Contract",
            None,
        )
        .unwrap()
        .to_string();

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, mock_instantiate, query).with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMsg {
        limit: 3,
        expiration: cw_utils::Expiration::AtTime(Timestamp::from_seconds(604800)),
        trade_shield_address,
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

    app.wasm_sudo(addr.clone(), &SudoMsg::ClockEndBlock {})
        .unwrap();

    // Query the contract for the existing order.
    let resp: GetLiquidAssetsResp = app
        .wrap()
        .query_wasm_smart(
            &addr,
            &QueryMsg::GetLiquidAssets {
                user_address: "user".to_string(),
            },
        )
        .unwrap();

    let mut expected: GetLiquidAssetsResp = GetLiquidAssetsResp {
        liquid_assets: vec![
            LiquidAsset {
                denom: "uelys".to_string(),
                price: Decimal::from_str("3.5308010067676894").unwrap(),
                available_amount: Decimal::from_str("45.666543").unwrap(),
                available_value: Decimal::from_str("161.239475999999978995").unwrap(),
                in_order_amount: Decimal::zero(),
                in_order_value: Decimal::zero(),
                total_amount: Decimal::from_str("45.666543").unwrap(),
                total_value: Decimal::from_str("161.239475999999978995").unwrap(),
            },
            LiquidAsset {
                denom: "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4"
                    .to_string(),
                price: Decimal::from_str("9.02450744362719844").unwrap(),
                available_amount: Decimal::from_str("37.403942").unwrap(),
                available_value: Decimal::from_str("337.552153000000000072").unwrap(),
                in_order_amount: Decimal::zero(),
                in_order_value: Decimal::zero(),
                total_amount: Decimal::from_str("37.403942").unwrap(),
                total_value: Decimal::from_str("337.552153000000000072").unwrap(),
            },
            LiquidAsset {
                denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                    .to_string(),
                price: Decimal::one(),
                available_amount: Decimal::from_str("5333229.342748").unwrap(),
                available_value: Decimal::from_str("5333229.342748").unwrap(),
                in_order_amount: Decimal::zero(),
                in_order_value: Decimal::zero(),
                total_amount: Decimal::from_str("5333229.342748").unwrap(),
                total_value: Decimal::from_str("5333229.342748").unwrap(),
            },
        ],
        total_liquid_asset_balance: DecCoin::new(
            Decimal256::zero(),
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
        ),
    };

    for i in expected.liquid_assets.iter() {
        expected.total_liquid_asset_balance.amount += Decimal256::from(i.total_value.clone());
    }

    // test if the response is the same as the expected
    assert_eq!(resp.liquid_assets.len(), expected.liquid_assets.len());

    assert_eq!(
        resp.liquid_assets
            .iter()
            .find(|l| l.denom.as_str() == "uelys")
            .cloned(),
        Some(expected.liquid_assets[0].clone())
    );
    assert_eq!(
        resp.liquid_assets
            .iter()
            .find(|l| l.denom.as_str()
                == "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4")
            .cloned(),
        Some(expected.liquid_assets[1].clone())
    );
    assert_eq!(
        resp.liquid_assets
            .iter()
            .find(|l| l.denom.as_str()
                == "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65")
            .cloned(),
        Some(expected.liquid_assets[2].clone())
    );
    assert_eq!(
        resp.total_liquid_asset_balance,
        expected.total_liquid_asset_balance
    );
}
