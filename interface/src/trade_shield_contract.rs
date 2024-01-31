use cw_orch::{interface, prelude::*};

use elys_bindings::trade_shield::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct TradeShieldContract;

impl<Chain: CwEnv> Uploadable for TradeShieldContract<Chain> {
    fn wasm(&self) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("trade_shield_contract")
            .unwrap()
    }

    // @TODO: use prelude types
    fn wrapper(&self) -> Box<dyn MockContract<Empty>> {
        Box::new(ContractWrapper::new_with_empty(
            trade_shield_contract::entry_point::execute,
            trade_shield_contract::entry_point::instantiate,
            trade_shield_contract::entry_point::query,
        ))
        .with_migrate(trade_shield_contract::entry_point::migrate)
    }
}
