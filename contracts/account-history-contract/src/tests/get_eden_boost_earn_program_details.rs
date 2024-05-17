use std::str::FromStr;

use crate::entry_point::instantiate;
use crate::{
    entry_point::{execute, query, sudo},
    msg::*,
};
use anyhow::{bail, Error, Result as AnyResult};
use cosmwasm_std::{
    coins, to_json_binary, Addr, Coin, Decimal, Empty, Int128, StdError, Timestamp, Uint128,
};
use cw_multi_test::{AppResponse, BasicAppBuilder, ContractWrapper, Executor, Module};
use elys_bindings::account_history::msg::query_resp::earn::GetEdenBoostEarnProgramResp;
use elys_bindings::account_history::types::earn_detail::earn_detail::AprEdenBoost;
use elys_bindings::account_history::types::earn_program::EdenBoostEarnProgram;
use elys_bindings::account_history::types::CoinValue;
use elys_bindings::query_resp::{
    BalanceBorrowed, DelegationDelegatorReward, EstakingRewardsResponse,
    MasterchefUserPendingRewardData, MasterchefUserPendingRewardResponse,
    QueryStableStakeAprResponse, StakedAvailable, Validator,
};
use elys_bindings::types::BalanceAvailable;
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::{
    ElysModule, ACCOUNT, ASSET_INFO, LAST_MODULE_USED, PERPETUAL_OPENED_POSITION, PRICES,
};
use trade_shield_contract::entry_point::{
    execute as trade_shield_execute, instantiate as trade_shield_init, query as trade_shield_query,
};
use trade_shield_contract::msg::InstantiateMsg as TradeShieldInstantiateMsg;

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
            ElysQuery::AmmBalance { address, denom } => {
                let resp = match (address.as_str(), denom.as_str()) {
                    ("user", "uedenb") => BalanceAvailable {
                        amount: Uint128::new(21798000),
                        usd_amount: Decimal::from_str("21798000").unwrap(),
                    },
                    (
                        "user",
                        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
                    ) => BalanceAvailable {
                        amount: Uint128::new(5333229342748),
                        usd_amount: Decimal::from_str("5333229342748").unwrap(),
                    },
                    _ => BalanceAvailable {
                        amount: Uint128::zero(),
                        usd_amount: Decimal::zero(),
                    },
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::MasterchefStableStakeApr { denom } => {
                let resp = match denom.as_str() {
                    "uusdc" => QueryStableStakeAprResponse {
                        apr: Int128::zero(),
                    },
                    "ueden" => QueryStableStakeAprResponse {
                        apr: Int128::zero(),
                    },
                    _ => return Err(Error::new(StdError::not_found(denom))),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::CommitmentStakedBalanceOfDenom { .. } => {
                let resp = StakedAvailable {
                    usd_amount: Decimal::from_atomics(Uint128::new(100130012), 3).unwrap(),
                    amount: Uint128::new(100120000000),
                    lockups: None,
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::StableStakeBalanceOfBorrow {} => {
                let resp = BalanceBorrowed {
                    usd_amount: Decimal::from_atomics(Uint128::new(3265035180871), 10).unwrap(),
                    percentage: Decimal::from_atomics(Uint128::new(0000238391578776388), 18)
                        .unwrap(),
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
            ElysQuery::AmmPriceByDenom { token_in, .. } => {
                let spot_price = match token_in.denom.as_str() {
                    "uelys" => Decimal::from_str("3.5308010067676894").unwrap(),
                    "ueden" => Decimal::from_str("3.5308010067676894").unwrap(),
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
            ElysQuery::EstakingRewards { .. } => {
                let resp = EstakingRewardsResponse {
                    rewards: vec![DelegationDelegatorReward {
                        validator_address: Validator::EdenBoost.to_string(),
                        reward: vec![Coin {
                            denom: "ueden".to_string(),
                            amount: Uint128::from_str("121").unwrap(),
                        }],
                    }],
                    total: vec![Coin {
                        denom: "uedenb".to_string(),
                        amount: Uint128::from_str("121").unwrap(),
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
fn get_eden_boost_earn_program_details() {
    // Create a wallet for the "user" with an initial balance of 100 usdc
    let wallet = vec![(
        "user",
        coins(
            100,
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
        ),
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
    let trade_shield_init = TradeShieldInstantiateMsg {
        account_history_address: None,
    };
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
    let code = ContractWrapper::new(execute, instantiate, query).with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMsg {
        limit: Some(3),
        expiration: Some(cw_utils::Expiration::AtTime(Timestamp::from_seconds(
            604800,
        ))),
        trade_shield_address: Some(trade_shield_address),
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

    let resp: GetEdenBoostEarnProgramResp = app
        .wrap()
        .query_wasm_smart(
            &addr,
            &QueryMsg::GetEdenBoostEarnProgramDetails {
                address: "user".to_string(),
            },
        )
        .unwrap();

    let expected = GetEdenBoostEarnProgramResp {
        data: EdenBoostEarnProgram {
            bonding_period: 0,
            apr: AprEdenBoost {
                uusdc: Uint128::zero(),
                ueden: Uint128::zero(),
            },
            available: Some(Uint128::new(21798000)),
            staked: Some(Uint128::new(100120000000)),
            rewards: Some(vec![CoinValue {
                denom: "ueden".to_string(),
                amount_token: Decimal::from_str("0.000121").unwrap(),
                price: Decimal::from_str("3.5308010067676894").unwrap(),
                amount_usd: Decimal::from_str("0.00042722692181889").unwrap(),
            }]),
        },
    };

    assert_eq!(resp, expected);
}
