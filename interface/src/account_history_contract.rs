use cw_orch::{interface, prelude::*};

use account_history_contract::msg::{InstantiateMsg, QueryMsg};

#[interface(InstantiateMsg, Empty, QueryMsg, Empty)]
pub struct AccountHistoryContract;

impl<Chain: CwEnv> Uploadable for AccountHistoryContract<Chain> {
    fn wasm(&self) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("account_history_contract")
            .unwrap()
    }

    // @TODO: use prelude types
    fn wrapper(&self) -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                account_history_contract::entry_point::execute,
                account_history_contract::entry_point::instantiate,
                account_history_contract::entry_point::query,
            )
            .with_migrate(account_history_contract::entry_point::migrate),
        )
    }
}
