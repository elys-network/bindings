use cosmwasm_std::StdError;

use crate::msg::query_resp::GetPerpetualOrderResp;

use super::*;

// This test case verifies that querying a non-existent order in the contract results in an "OrderNotFound" error.
#[test]
fn not_found() {
    // Initialize the ElysApp instance.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        perpetual_orders: vec![],
    };

    // Define an order ID that does not exist in the contract (e.g., 0).
    let id: u64 = 0;

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id: u64 = app.store_code(Box::new(code));

    // Instantiate the contract with "owner" as the deployer.
    let addr: Addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();

    // Query the contract for the non-existent order and expect an "OrderNotFound" error.
    let err: StdError = app
        .wrap()
        .query_wasm_smart::<GetPerpetualOrderResp>(&addr, &QueryMsg::GetPerpetualOrder { id })
        .unwrap_err();

    // Define the expected error message.
    let error_reference = StdError::GenericErr {
        msg: format!(
            "Querier contract error: {}",
            ContractError::OrderNotFound { order_id: id }.to_string()
        ),
    };

    // Verify that the error message matches the expected error.
    assert_eq!(err, error_reference);
}
