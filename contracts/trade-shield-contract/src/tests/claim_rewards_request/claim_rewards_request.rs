use std::collections::HashMap;
use std::str::FromStr;
use std::vec;

use super::instantiate;
use super::InstantiateMockMsg;
use crate::entry_point::{execute, query};
use anyhow::{bail, Result as AnyResult};
use cosmwasm_std::coin;
use cosmwasm_std::Coin;
use cosmwasm_std::DecCoin;
use cosmwasm_std::Decimal256;
use cosmwasm_std::StdError;
use cosmwasm_std::Uint128;
use cosmwasm_std::Uint256;
use cosmwasm_std::{to_json_binary, Addr, Empty};
use cw_multi_test::BankSudo;
use cw_multi_test::Executor;
use cw_multi_test::SudoMsg;
use cw_multi_test::{AppResponse, BasicAppBuilder, ContractWrapper, Module};
use cw_storage_plus::Item;
use cw_storage_plus::Map;
use elys_bindings::query_resp::DelegationDelegatorReward;
use elys_bindings::query_resp::EstakingRewardsResponse;
use elys_bindings::query_resp::MasterchefUserPendingRewardData;
use elys_bindings::query_resp::MasterchefUserPendingRewardResponse;
use elys_bindings::query_resp::Validator;
use elys_bindings::trade_shield::msg::ExecuteMsg;
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::ElysModule;

const DENOM_INFO: [(&str, u32); 2] = [
    (
        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
        6,
    ),
    ("uelys", 6),
];

const ESTAKING_REWARD: Map<&str, Vec<(String, Vec<DecCoin>)>> = Map::new("estaking reward");
const MASTER_USER_PENDING_REWARD: Map<&str, Vec<(u64, Vec<Coin>)>> =
    Map::new("master user pending reward");
const MSG_CALLED: Item<Vec<ElysMsg>> = Item::new("msg called");

struct ElysModuleWrapper(ElysModule);

impl Module for ElysModuleWrapper {
    type QueryT = ElysQuery;
    type ExecT = ElysMsg;
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
            ElysQuery::EstakingRewards { address } => {
                let rewards = ESTAKING_REWARD.load(storage, &address)?;

                let delegation_reward: Vec<DelegationDelegatorReward> = rewards
                    .iter()
                    .map(|(validator, rewards)| DelegationDelegatorReward {
                        validator_address: validator.to_string(),
                        reward: rewards.to_owned(),
                    })
                    .collect();
                let mut map: HashMap<&str, Decimal256> = HashMap::new();

                for (_, deccoins) in rewards.iter() {
                    for deccoin in deccoins.iter() {
                        let deccoin_entry = map
                            .entry(deccoin.denom.as_str())
                            .or_insert(Decimal256::zero());
                        *deccoin_entry += deccoin.amount.clone();
                    }
                }

                let mut total: Vec<DecCoin> = vec![];

                for (denom, amount) in map {
                    total.push(DecCoin::new(amount, denom));
                }

                let resp = EstakingRewardsResponse {
                    rewards: delegation_reward,
                    total,
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::MasterchefUserPendingReward { user } => {
                let pool_rewards = MASTER_USER_PENDING_REWARD.load(storage, user.as_str())?;

                let rewards: Vec<MasterchefUserPendingRewardData> = pool_rewards
                    .iter()
                    .map(|(pool_id, reward)| MasterchefUserPendingRewardData {
                        pool_id: pool_id.to_owned(),
                        reward: reward.to_owned(),
                    })
                    .collect();

                let mut map: HashMap<&str, u128> = HashMap::new();
                for (_, amounts) in pool_rewards.iter() {
                    for amount in amounts.iter() {
                        let entry_reward = map.entry(amount.denom.as_str()).or_insert(0);
                        *entry_reward += amount.amount.u128();
                    }
                }
                let mut total_rewards: Vec<Coin> = vec![];
                for (denom, amount) in map {
                    total_rewards.push(coin(amount, denom));
                }

                let resp = MasterchefUserPendingRewardResponse {
                    rewards,
                    total_rewards,
                };
                Ok(to_json_binary(&resp)?)
            }
            _ => panic!("not implemented {request:?}"),
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
        let mut msgs_called = MSG_CALLED.load(storage)?;
        msgs_called.push(msg.clone());
        MSG_CALLED.save(storage, &msgs_called)?;

        match msg {
            ElysMsg::EstakingWithdrawElysStakingRewards { .. } => Ok(AppResponse::default()),
            ElysMsg::EstakingWithdrawReward {
                validator_address,
                delegator_address,
            } => {
                let mut rewards = ESTAKING_REWARD.load(storage, delegator_address.as_str())?;
                let reward_found = if let Some(index) = rewards
                    .iter_mut()
                    .position(|reward| reward.0 == validator_address)
                {
                    let reward_found: Vec<Coin> = rewards[index]
                        .1
                        .iter()
                        .map(|reward| {
                            coin(
                                Uint128::from_str(reward.amount.atomics().to_string().as_str())
                                    .unwrap()
                                    .u128()
                                    / 10u128.pow(
                                        Decimal256::DECIMAL_PLACES
                                            - DENOM_INFO
                                                .iter()
                                                .find(|denom_info| denom_info.0 == reward.denom)
                                                .unwrap()
                                                .1,
                                    ),
                                reward.denom.as_str(),
                            )
                        })
                        .collect();
                    rewards.remove(index);
                    reward_found
                } else {
                    return Err(StdError::generic_err(format!(
                        "reward validator not found: [{}]",
                        validator_address,
                    ))
                    .into());
                };
                ESTAKING_REWARD.save(storage, delegator_address.as_str(), &rewards)?;
                router.sudo(
                    api,
                    storage,
                    block,
                    SudoMsg::Bank(BankSudo::Mint {
                        to_address: delegator_address,
                        amount: reward_found,
                    }),
                )
            }
            ElysMsg::MasterchefClaimRewards { sender, pool_ids } => {
                let mut pools_reward = MASTER_USER_PENDING_REWARD.load(storage, sender.as_str())?;
                let mut rewards_found: Vec<Coin> = vec![];
                for id in pool_ids {
                    if let Some(index) = pools_reward.iter_mut().position(|reward| reward.0 == id) {
                        rewards_found.append(&mut pools_reward[index].1.clone());
                        pools_reward.remove(index);
                    } else {
                        return Err(StdError::generic_err(format!(
                            "reward not found for pool_id: {id}"
                        ))
                        .into());
                    }
                }
                router.sudo(
                    api,
                    storage,
                    block,
                    SudoMsg::Bank(BankSudo::Mint {
                        to_address: sender,
                        amount: rewards_found,
                    }),
                )
            }
            _ => bail!("not implemented {msg:?}"),
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
fn claim_rewards_request() {
    let mut app = BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
        .with_custom(ElysModuleWrapper(ElysModule {}))
        .build(|_roouter, _, storage| {
            ESTAKING_REWARD
                .save(
                    storage,
                    "user",
                    &vec![(
                        Validator::Eden.to_string(),
                        vec![DecCoin::new(
                            Decimal256::from_atomics(Uint256::from_u128(24100000), 6).unwrap(),
                            "uelys",
                        )],
                    )],
                )
                .unwrap();

            let rewards: Vec<(u64, Vec<Coin>)> =
                vec![(1, vec![]), (2, vec![coin(500, DENOM_INFO[0].0)])];

            MASTER_USER_PENDING_REWARD
                .save(storage, "user", &rewards)
                .unwrap();
            MSG_CALLED.save(storage, &vec![]).unwrap();
        });

    let code = ContractWrapper::new(execute, instantiate, query);

    let code_id = app.store_code(Box::new(code));

    let instantiate_msg = InstantiateMockMsg {
        account_history_address: None,
        spot_orders: vec![],
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

    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::ClaimRewardsRequest {},
        &[],
    )
    .unwrap();

    let msgs_called = app.init_modules(|_, _, storage| MSG_CALLED.load(storage).unwrap());

    assert_eq!(msgs_called.len(), 3);
    assert_eq!(
        msgs_called[0],
        ElysMsg::estaking_withdraw_elys_staking_rewards("user".to_string())
    );
    assert_eq!(
        msgs_called[1],
        ElysMsg::estaking_withdraw_reward("user".to_string(), Validator::Eden.to_string())
    );
    assert_eq!(
        msgs_called[2],
        ElysMsg::get_masterchef_claim_rewards("user".to_string(), vec![2])
    );

    let user_usdc_balance = app.wrap().query_balance("user", DENOM_INFO[0].0).unwrap();
    let user_uelys_balance = app.wrap().query_balance("user", "uelys").unwrap();

    assert_eq!(user_usdc_balance, coin(500, DENOM_INFO[0].0));
    assert_eq!(user_uelys_balance, coin(24100000, "uelys"));
}
