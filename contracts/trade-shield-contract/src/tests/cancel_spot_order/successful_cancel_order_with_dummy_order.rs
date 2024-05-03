use cancel_spot_order::test_order_status::test_spot_order_status;

use super::*;

// This test case verifies a successful order cancellation in the contract using a dummy order.
#[test]
fn successful_cancel_order_with_dummy_order() {
    // Create wallets for "user" and "owner" with initial balances.
    let wallets = vec![("user", vec![]), ("owner", coins(1200, "btc"))];

    // Initialize the ElysApp instance with the specified wallets.
    let mut app = ElysApp::new_with_wallets(wallets);

    // Create a dummy order to be used for instantiation. (with an amount of 1000btc)
    let dummy_order = SpotOrder::new_dummy();

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        account_history_address: None,
        spot_orders: vec![dummy_order.clone()],
        perpetual_orders: vec![],
    };

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    // Instantiate the contract, providing "owner" with an initial balance of 1200 BTC.
    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(1200, "btc"),
            "Contract",
            None,
        )
        .unwrap();

    test_spot_order_status(
        &app,
        addr.to_string(),
        dummy_order.order_id,
        Status::Pending,
    );

    // User "user" cancels the dummy order.
    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CancelSpotOrder {
            order_id: dummy_order.order_id,
        },
        &[],
    )
    .unwrap();

    test_spot_order_status(
        &app,
        addr.to_string(),
        dummy_order.order_id,
        Status::Canceled,
    );

    // Verify that the "user" now has a balance of 1000 BTC, and the contract address has 200 BTC.
    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        1000
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        200
    );
}
