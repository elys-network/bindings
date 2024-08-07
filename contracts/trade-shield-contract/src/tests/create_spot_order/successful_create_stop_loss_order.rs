use crate::tests::get_order_id_from_events::get_order_id_from_events;

use super::*;

// This test case verifies the successful creation of a "stop-loss" order in the contract.
// A "stop-loss" order is designed to protect against losses. In a "stop-loss" order, the user specifies a trigger price.
// If the market price of BTC falls to or below the trigger price, the order is executed as a "market sell" to limit potential losses.
#[test]
fn successful_create_stop_loss_order() {
    // Create a wallet for the "user" with an initial balance of 2 BTC.
    let wallet = vec![("user", coins(2, "btc"))];

    // Initialize the ElysApp instance with the specified wallet.
    let mut app = ElysApp::new_with_wallets(wallet);

    let prices = vec![
        Price::new("btc", Decimal::from_str("30000.0").unwrap()),
        Price::new("usdc", Decimal::from_str("1.0").unwrap()),
    ];
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices).unwrap());

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
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

    // User "user" creates a "stop-loss" order for BTC, specifying the trigger price in USDC.
    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateSpotOrder {
                order_type: SpotOrderType::StopLoss,
                order_price: Some(OrderPrice {
                    base_denom: "btc".to_string(),
                    quote_denom: "usdc".to_string(),
                    rate: Decimal::from_atomics(Uint128::new(30000), 0).unwrap(), // The trigger price of 30000 USDC per BTC.
                }),

                order_source_denom: "btc".to_string(),
                order_target_denom: "usdc".to_string(),
            },
            &coins(2, "btc"), // User's BTC balance.
        )
        .unwrap();

    // Verify that the "user" no longer has any BTC after creating the order.
    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    // Verify that the contract address now holds the 2 BTC for the order (market sell).
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        2
    );

    // Verify that an order ID is emitted in the contract's events.
    assert!(get_order_id_from_events(&resp.events).is_some());
}
