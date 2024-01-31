use cw_orch::{interface, prelude::*};

use financial_snapshot_contract::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct FinancialSnapshotContract;

impl<Chain: CwEnv> Uploadable for FinancialSnapshotContract<Chain> {
    fn wasm(&self) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("financial_snapshot_contract")
            .unwrap()
    }

    // @TODO: use prelude types
    fn wrapper(&self) -> Box<dyn MockContract<Empty>> {
        Box::new(ContractWrapper::new_with_empty(
            None,
            financial_snapshot_contract::instantiate,
            financial_snapshot_contract::query,
        ))
    }
}
