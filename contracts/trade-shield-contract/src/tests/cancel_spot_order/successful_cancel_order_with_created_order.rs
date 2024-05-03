use super::*;
use cancel_spot_order::test_order_status::test_spot_order_status;
use get_order_id_from_events::get_order_id_from_events;

// This test case verifies the successful cancellation of a created order in the contract.
#[test]
fn successful_cancel_order_with_created_order() {
    // Create a wallet for the "user" with an initial balance of 150 ETH.
    let wallets = vec![("user", coins(150, "eth"))];

    // Initialize the ElysApp instance with the specified wallets.
    let mut app = ElysApp::new_with_wallets(wallets);

    let prices = vec![
        Price::new("btc", Decimal::from_str("30000.0").unwrap()),
        Price::new("eth", Decimal::from_str("2040.0").unwrap()),
    ];
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices).unwrap());

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        account_history_address: None,
        spot_orders: vec![],
        perpetual_orders: vec![],
    };

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query);
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

    // User "user" creates an order in the contract.
    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateSpotOrder {
                order_type: SpotOrderType::StopLoss,
                order_price: Some(OrderPrice {
                    rate: Decimal::from_atomics(Uint128::new(18), 0).unwrap(),
                    base_denom: "eth".to_string(),
                    quote_denom: "btc".to_string(),
                }),
                order_source_denom: "eth".to_owned(),
                order_target_denom: "btc".to_string(),
            },
            &coins(45, "eth"),
        )
        .unwrap();

    // Retrieve the order ID from the events emitted during order creation.
    let id = get_order_id_from_events(&resp.events).unwrap();

    test_spot_order_status(&app.wrap(), addr.to_string(), id, Status::Pending);

    // User "user" cancels the created order.
    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CancelSpotOrder { order_id: id },
        &[],
    )
    .unwrap();

    test_spot_order_status(&app.wrap(), addr.to_string(), id, Status::Canceled);

    // Verify that the "user" now has a balance of 150 ETH, and the contract address has 0 ETH.
    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        150
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        0
    );
}
