use std::str::FromStr;

use crate::states::{EXPIRATION, PAGINATION, TRADE_SHIELD_ADDRESS, VALUE_DENOM};
use crate::tests::get_staked_assets::query_resp::StakedAssetsResponse;
use crate::types::earn_program::{
    EdenBoostEarnProgram, EdenEarnProgram, ElysEarnProgram, UsdcEarnProgram,
};
use crate::types::{AprElys, AprUsdc, BalanceReward, StakedAssets};
use crate::{
    entry_point::{execute, query, sudo},
    msg::*,
};
use anyhow::{bail, Error, Result as AnyResult};
use cosmwasm_std::{
    coins, to_json_binary, Addr, DecCoin, Decimal, Decimal256, DepsMut, Empty, Env, Int128,
    MessageInfo, Response, StdError, StdResult, Timestamp, Uint128,
};
use cw_multi_test::{AppResponse, BasicAppBuilder, ContractWrapper, Executor, Module};
use elys_bindings::query_resp::{
    BalanceBorrowed, Entry, Lockup, QueryGetEntryResponse, StakedAvailable,
};
use elys_bindings::types::{
    BalanceAvailable, OracleAssetInfo, PageRequest, Price, StakedPosition, StakingValidator,
    UnstakedPosition,
};
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::{
    ElysModule, ACCOUNT, ASSET_INFO, LAST_MODULE_USED, MARGIN_OPENED_POSITION, PRICES,
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
    VALUE_DENOM.save(deps.storage, &msg.value_denom)?;
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
                            authority: "".to_string(),
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
                    _ => return Err(Error::new(StdError::not_found(base_denom))),
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
fn get_staked_assets() {
    // Create a wallet for the "user" with an initial balance of 100 usdc
    let wallet = vec![(
        "user",
        coins(
            200__000_000,
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
        ),
    )];

    let prices: Vec<Price> = vec![
        Price::new("uelys", Decimal::from_str("1.5").unwrap()),
        Price::new("uusdc", Decimal::from_str("1.0").unwrap()),
    ];

    let infos = vec![
        OracleAssetInfo::new(
            "uusdc".to_string(),
            "UUSDC".to_string(),
            "".to_string(),
            "".to_string(),
            6,
        ),
        OracleAssetInfo::new(
            "uelys".to_string(),
            "UELYS".to_string(),
            "".to_string(),
            "".to_string(),
            6,
        ),
        OracleAssetInfo::new(
            "ueden".to_string(),
            "UEDEN".to_string(),
            "".to_string(),
            "".to_string(),
            6,
        ),
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
            MARGIN_OPENED_POSITION.save(storage, &vec![]).unwrap();
            ASSET_INFO.save(storage, &vec![]).unwrap();
            PRICES.save(storage, &vec![]).unwrap();
            LAST_MODULE_USED.save(storage, &None).unwrap();
        });

    app.init_modules(|router, _, store| {
        router.custom.0.set_prices(store, &prices).unwrap();
        router.custom.0.set_asset_infos(store, &infos)
    })
    .unwrap();

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
        value_denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            .to_string(),
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
    let resp: StakedAssetsResponse = app
        .wrap()
        .query_wasm_smart(
            &addr,
            &QueryMsg::GetStakedAssets {
                user_address: "user".to_string(),
            },
        )
        .unwrap();

    let expected: StakedAssetsResponse = StakedAssetsResponse {
        total_staked_balance: DecCoin::new(
            Decimal256::from_str("774.00319452761366262").unwrap(),
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
        ),
        staked_assets: StakedAssets {
            eden_boost_earn_program: EdenBoostEarnProgram {
                bonding_period: 0,
                apr: AprUsdc {
                    uusdc: Uint128::zero(),
                    ueden: Uint128::new(29),
                },
                available: Some(Uint128::zero()),
                staked: Some(Uint128::zero()),
                rewards: Some(vec![
                    BalanceReward {
                        asset: "uusdc".to_string(),
                        amount: Uint128::zero(),
                        usd_amount: Some(Decimal::zero()),
                    },
                    BalanceReward {
                        asset: "ueden".to_string(),
                        amount: Uint128::zero(),
                        usd_amount: Some(Decimal::zero()),
                    },
                ]),
            },
            eden_earn_program: EdenEarnProgram {
                bonding_period: 0,
                apr: AprElys {
                    uusdc: Uint128::zero(),
                    ueden: Uint128::new(29),
                    uedenb: Uint128::new(100),
                },
                available: Some(BalanceAvailable {
                    amount: Uint128::zero(),
                    usd_amount: Decimal::zero(),
                }),
                staked: Some(StakedAvailable {
                    usd_amount: Decimal::from_str("771.023521368147227293").unwrap(),
                    amount: Uint128::new(2587611057),
                    lockups: Some(vec![Lockup {
                        amount: Int128::new(5200770174),
                        unlock_timestamp: 1706538974,
                    }]),
                }),
                rewards: Some(vec![
                    BalanceReward {
                        asset: "uusdc".to_string(),
                        amount: Uint128::new(432),
                        usd_amount: Some(Decimal::from_str("0.0004320864").unwrap()),
                    },
                    BalanceReward {
                        asset: "ueden".to_string(),
                        amount: Uint128::new(1854860),
                        usd_amount: Some(Decimal::from_str("0.552687655656791223").unwrap()),
                    },
                    BalanceReward {
                        asset: "uedenb".to_string(),
                        amount: Uint128::new(6310541),
                        usd_amount: None,
                    },
                ]),
                vesting: Some(BalanceAvailable {
                    amount: Uint128::zero(),
                    usd_amount: Decimal::zero(),
                }),
                vesting_details: Some(vec![]), // FIXME: according to Wari we should have vesting details here
            },
            elys_earn_program: ElysEarnProgram {
                bonding_period: 14,
                apr: AprElys {
                    uusdc: Uint128::zero(),
                    ueden: Uint128::new(29),
                    uedenb: Uint128::new(100),
                },
                available: Some(BalanceAvailable {
                    amount: Uint128::new(49774186),
                    usd_amount: Decimal::from_str("14.831080605849001273").unwrap(),
                }),
                staked: Some(StakedAvailable {
                    usd_amount: Decimal::from_str("2.979673159466435327").unwrap(),
                    amount: Uint128::new(10000000),
                    lockups: Some(vec![]),
                }),
                rewards: Some(vec![
                    BalanceReward {
                        asset: "uusdc".to_string(),
                        amount: Uint128::zero(),
                        usd_amount: Some(Decimal::zero()),
                    },
                    BalanceReward {
                        asset: "ueden".to_string(),
                        amount: Uint128::new(6152),
                        usd_amount: Some(Decimal::from_str("0.001833094927703751").unwrap()),
                    },
                    BalanceReward {
                        asset: "uedenb".to_string(),
                        amount: Uint128::new(654069181),
                        usd_amount: None,
                    },
                ]),
                staked_positions: Some(vec![
                    // FIXME: We want to remove the item below as amount is zero
                    // StakedPosition {
                    //     id: "1".to_string(),
                    //     validator: StakingValidator {
                    //         address: "elysvaloper1q228fz8ctu59udlpf5xmdhyahwdmvlwd2x9m6m"
                    //             .to_string(),
                    //         name: "F5 Nodes".to_string(),
                    //         voting_power: Decimal::from_str("0.3472745336338554").unwrap(),
                    //         commission: Decimal::from_str("0.05").unwrap(),
                    //         profile_picture_src: Some("https://f5nodes.com".to_string()),
                    //     },
                    //     staked: BalanceAvailable {
                    //         amount: Uint128::zero(),
                    //         usd_amount: Decimal::zero(),
                    //     },
                    // },
                    StakedPosition {
                        id: "2".to_string(),
                        validator: StakingValidator {
                            address: "elysvaloper1ng8sen6z5xzcfjtyrsedpe43hglymq040x3cpw"
                                .to_string(),
                            name: "nirvana".to_string(),
                            voting_power: Decimal::from_str("25.6521469796402094").unwrap(),
                            commission: Decimal::from_str("0.1").unwrap(),
                            profile_picture_src: Some("https://elys.network".to_string()),
                        },
                        staked: BalanceAvailable {
                            amount: Uint128::new(10000000),
                            usd_amount: Decimal::from_str("2.979673159466435327").unwrap(),
                        },
                    },
                ]),
                unstaked_positions: Some(vec![UnstakedPosition {
                    id: "1".to_string(),
                    validator: StakingValidator {
                        address: "elysvaloper1ng8sen6z5xzcfjtyrsedpe43hglymq040x3cpw".to_string(),
                        name: "nirvana".to_string(),
                        voting_power: Decimal::from_str("25.6521469796402094").unwrap(),
                        commission: Decimal::from_str("0.1").unwrap(),
                        profile_picture_src: Some("https://elys.network".to_string()),
                    },
                    remaining_time: 1707328694000,
                    unstaked: BalanceAvailable {
                        amount: Uint128::new(100038144098),
                        usd_amount: Decimal::from_str("29808.097289164619005282").unwrap(),
                    },
                }]),
            },
            usdc_earn_program: UsdcEarnProgram {
                bonding_period: 0,
                apr: AprUsdc {
                    uusdc: Uint128::new(100),
                    ueden: Uint128::new(168),
                },
                available: Some(BalanceAvailable {
                    amount: Uint128::new(5333264347748),
                    usd_amount: Decimal::from_str("5334331.0006175496").unwrap(),
                }),
                staked: Some(StakedAvailable {
                    usd_amount: Decimal::zero(),
                    amount: Uint128::zero(),
                    lockups: None,
                }),
                rewards: Some(vec![
                    BalanceReward {
                        asset: "uusdc".to_string(),
                        amount: Uint128::zero(),
                        usd_amount: Some(Decimal::zero()),
                    },
                    BalanceReward {
                        asset: "ueden".to_string(),
                        amount: Uint128::new(247665114),
                        usd_amount: Some(Decimal::from_str("73.796109272199488447").unwrap()),
                    },
                ]),
                borrowed: Some(BalanceBorrowed {
                    usd_amount: Decimal::from_str("204040.8000010002").unwrap(),
                    percentage: Decimal::one(),
                }),
            },
        },
    };

    // test if the response is the same as the expected
    assert_eq!(resp, expected);
}
