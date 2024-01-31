use cosmwasm_std::Int128;

use super::*;

#[test]
fn closing_perpetualg_position() {
    // Initialize the ElysApp.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        perpetual_orders: vec![],
    };

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
    let code_id = app.store_code(Box::new(code));

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

    app.execute_contract(
        Addr::unchecked("user"),
        addr,
        &ExecuteMsg::ClosePerpetualPosition {
            id: 2,
            amount: Int128::new(300),
        },
        &[],
    )
    .unwrap();

    let last_module_used = app
        .init_modules(|router, _, storage| router.custom.get_last_module(storage).unwrap())
        .unwrap();
    assert_eq!(last_module_used, "PerpetualClose");
}
