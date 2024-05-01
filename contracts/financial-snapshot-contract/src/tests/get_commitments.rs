use anyhow::{bail, Result as AnyResult};
use cosmwasm_std::{to_json_binary, Addr, Empty, Int128};
use cw_multi_test::{AppResponse, Module};
use cw_multi_test::{BasicAppBuilder, ContractWrapper, Executor};
use elys_bindings::query_resp::{
    CommitmentsRaw, QueryShowCommitmentsResponse, QueryShowCommitmentsResponseRaw, VestingTokens,
    VestingTokensRaw,
};
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::ElysModule;

use crate::entry_point::query;
use crate::QueryMsg;

use super::mock::execute::execute;
use super::mock::instantiate;

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
            ElysQuery::CommitmentShowCommitments { creator } => {
                Ok(to_json_binary(&QueryShowCommitmentsResponseRaw {
                    commitments: CommitmentsRaw {
                        creator,
                        committed_tokens: None,
                        rewards_unclaimed: None,
                        claimed: None,
                        vesting_tokens: Some(vec![VestingTokensRaw {
                            denom: "uelys".to_string(),
                            total_amount: Int128::new(2000),
                            num_blocks: None,
                            start_block: None,
                            vest_started_timestamp: Some(8),
                            claimed_amount: Int128::zero(),
                        }]),
                        rewards_by_elys_unclaimed: None,
                        rewards_by_eden_unclaimed: None,
                        rewards_by_edenb_unclaimed: None,
                        rewards_by_usdc_unclaimed: None,
                    },
                })?)
            }
            _ => panic!("not implemented"),
        }
    }

    fn execute<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _sender: Addr,
        _msg: Self::ExecT,
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
        bail!("execute is not implemented for ElysModule")
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
fn get_commitments_missing_field() {
    let mut app = BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
        .with_custom(ElysModuleWrapper(ElysModule {}))
        .build(|_, _, _| {});

    let code = ContractWrapper::new(execute, instantiate::instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate::InstantiateMockMsg {
                epoch_cycle_interval: 0,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: QueryShowCommitmentsResponse = app
        .wrap()
        .query_wasm_smart(
            addr.as_str(),
            &QueryMsg::GetCommitments {
                delegator_addr: "user".to_string(),
            },
        )
        .unwrap();
    let vesting: VestingTokens = resp.commitments.vesting_tokens.unwrap()[0].clone();
    let vesting_dummy: VestingTokens = VestingTokens {
        denom: "uelys".to_string(),
        total_amount: Int128::new(2000),
        vest_started_timestamp: 8,
        claimed_amount: Int128::zero(),
        num_blocks: 0,
        start_block: 0,
    };

    assert_eq!(vesting, vesting_dummy);
}
