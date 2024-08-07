use crate::tests::get_order_id_from_events::get_order_id_from_events;

use super::*;
// This test case verifies the successful creation of a "limit buy" order in the contract.
// In a "limit buy" order, the user specifies the maximum price (rate) they are willing to pay for BTC in terms of USDC.
// If the market price of BTC falls to or below the specified rate, the order is executed.
#[test]
fn successful_create_limit_buy_order() {
    // Create a wallet for the "user" with an initial balance of 100 USDC.
    let wallet = vec![("user", coins(100, "usdc"))];

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

    // User "user" creates a "limit buy" order for BTC, specifying the maximum price in USDC.
    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateSpotOrder {
                order_type: SpotOrderType::LimitBuy,
                order_price: Some(OrderPrice {
                    base_denom: "usdc".to_string(),
                    quote_denom: "btc".to_string(),
                    rate: Decimal::one()
                        .checked_div(Decimal::from_str("30000").unwrap())
                        .unwrap(), // The maximum price of 30000 USDC per BTC.
                }),
                order_source_denom: "usdc".to_string(),
                order_target_denom: "btc".to_string(),
            },
            &coins(100, "usdc"), // User's USDC balance.
        )
        .unwrap();

    // Verify that the "user" no longer has any USDC after creating the order.
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    // Verify that the contract address now holds the 100 USDC for the order.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        100
    );

    // Verify that an order ID is emitted in the contract's events.
    assert!(get_order_id_from_events(&resp.events).is_some());
}
