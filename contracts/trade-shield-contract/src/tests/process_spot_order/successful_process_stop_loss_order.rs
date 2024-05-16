use super::*;
use cosmwasm_std::{coins, BlockInfo, Coin, Timestamp};
use process_spot_order::test_order_status::test_spot_order_status;
// This test case verifies the successful processing of a "stop-loss" order in the contract.
// The scenario involves a "stop-loss" order created by a user to protect against a decline in BTC price.
// - Initially, the BTC price is 30,000 USDC, and the trigger price in the order is 20,000 USDC.
// - The order is created with 2 BTC to sell if the trigger price is reached.
// - After processing the order, the BTC price drops to 20,000 USDC, triggering the order.
// - The order is executed as a "market sell," and the user receives 40,000 USDC.
#[test]
fn successful_process_stop_loss_order() {
    // Initialize the ElysApp instance with wallets for "owner" and "user."
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("owner", coins(2, "btc")), ("user", vec![])];
    let mut app = ElysApp::new_with_wallets(wallets);

    // Define BTC and USDC prices at two different time intervals (t0 and t1).
    let prices_at_t0 = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(30000), 0).unwrap(),
        ),
        Price::new(
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
            Decimal::from_atomics(Uint128::new(1), 0).unwrap(),
        ),
    ];
    let prices_at_t1 = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(20000), 0).unwrap(),
        ),
        Price::new(
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
            Decimal::from_atomics(Uint128::new(1), 0).unwrap(),
        ),
    ];

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query)
        .with_reply(reply)
        .with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    // Create a "stop-loss" order (dummy order) with trigger price and balance.
    let dummy_order = SpotOrder::new(
        0,
        SpotOrderType::StopLoss,
        Some(OrderPrice {
            base_denom: "btc".to_string(),
            quote_denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                .to_string(),
            rate: Decimal::from_str("20000").unwrap(), // Trigger price of 20,000 USDC per BTC.
        }),
        coin(2, "btc"), // 2 BTC to be sold.
        Addr::unchecked("user"),
        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
        &BlockInfo {
            height: 50,
            time: Timestamp::from_seconds(600),
            chain_id: "elys-app".to_string(),
        },
    );

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        account_history_address: None,
        spot_orders: vec![dummy_order.clone()],
        perpetual_orders: vec![],
    };

    // Create an sudo message to process orders.
    let sudo_msg = SudoMsg::ClockEndBlock {};

    // Instantiate the contract with "owner" as the deployer and deposit 2 BTC.
    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(2, "btc"),
            "Contract",
            None,
        )
        .unwrap();

    // Set the initial BTC and USDC prices.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t0))
        .unwrap();

    test_spot_order_status(
        &app.wrap(),
        addr.to_string(),
        dummy_order.order_id,
        Status::Pending,
    );

    // Execute the order processing.
    app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

    test_spot_order_status(
        &app.wrap(),
        addr.to_string(),
        dummy_order.order_id,
        Status::Pending,
    );

    // Verify the resulting balances after order processing.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        2
    );
    assert_eq!(
        app.wrap()
            .query_balance(
                &addr,
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            )
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance(
                "user",
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            )
            .unwrap()
            .amount
            .u128(),
        0
    );

    // Update the BTC and USDC prices to trigger the order.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t1))
        .unwrap();

    // Execute the order processing.
    app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

    test_spot_order_status(
        &app.wrap(),
        addr.to_string(),
        dummy_order.order_id,
        Status::Executed,
    );

    // Verify the resulting balances after order processing.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance(
                &addr,
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            )
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance(
                "user",
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            )
            .unwrap()
            .amount
            .u128(),
        40000 // User receives 40,000 USDC from the executed "stop-loss" order.
    );
}
