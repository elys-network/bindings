use super::*;
use cosmwasm_std::{Addr, Coin, Decimal, Int128, SignedDecimal, SignedDecimal256};

#[test]
fn successful_process_limit_buy_order() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![];
    let mut app = ElysApp::new_with_wallets(wallets);

    // Define ubtc and ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65 prices at two different time intervals (t0 and t1).
    let prices_at_t0 = vec![
        Price::new("ubtc", Decimal::from_atomics(Uint128::new(70), 0).unwrap()),
        Price::new(
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
            Decimal::from_atomics(Uint128::new(1), 0).unwrap(),
        ),
    ];
    let prices_at_t1 = vec![
        Price::new("ubtc", Decimal::from_atomics(Uint128::new(30), 0).unwrap()),
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

    // Create a "limit open" order (dummy order) with a specific rate and balance.
    let dummy_order = PerpetualOrder::new_close(
        "user",
        1,
        &PerpetualOrderType::LimitClose,
        &coin(
            120,
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
        ),
        "ubtc",
        &SignedDecimal::from_str("1.1").unwrap(),
        1,
        &Some(OrderPrice {
            base_denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                .to_string(),
            quote_denom: "ubtc".to_string(),
            rate: Decimal::one()
                .checked_div(Decimal::from_atomics(Uint128::new(38), 0).unwrap())
                .unwrap(), // Rate at which ubtc will be bought (38 ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65 per ubtc).
        }),
        &Some(SignedDecimal256::from_str("1.1").unwrap()),
        &vec![],
    )
    .unwrap();

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        perpetual_orders: vec![dummy_order],
    };

    // Create an sudo message to process orders.
    let sudo_msg = SudoMsg::ClockEndBlock {};

    // Instantiate the contract with "owner" as the deployer and deposit 120 ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65.
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

    app.init_modules(|router, _, storage| {
        router.custom.set_mtp(
            storage,
            &vec![Mtp {
                address: "user".to_string(),
                liabilities: Int128::zero(),
                take_profit_liabilities: Int128::zero(),
                mtp_health: SignedDecimal::one(),
                position: 2,
                id: 1,
                amm_pool_id: 1,
                consolidate_leverage: SignedDecimal::zero(),
                sum_collateral: Int128::zero(),
                take_profit_price: SignedDecimal256::from_str("1.2").unwrap(),
                borrow_interest_paid_collateral: Int128::zero(),
                borrow_interest_paid_custody: Int128::zero(),
                borrow_interest_unpaid_collateral: Int128::zero(),
                collateral_asset: "btc".to_string(),
                collateral: Int128::zero(),
                custody: Int128::zero(),
                custody_asset: "btc".to_string(),
                funding_fee_paid_collateral: Int128::zero(),
                funding_fee_paid_custody: Int128::zero(),
                funding_fee_received_collateral: Int128::zero(),
                funding_fee_received_custody: Int128::zero(),
                leverage: SignedDecimal::one(),
                liabilities_asset:
                    "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                        .to_string(),
                open_price: SignedDecimal::one(),
                take_profit_borrow_rate: SignedDecimal::one(),
                take_profit_custody: Int128::zero(),
                trading_asset:
                    "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                        .to_string(),
            }],
        )
    })
    .unwrap();

    // Set the initial ubtc and ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65 prices.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t0))
        .unwrap();

    // Execute the order processing.
    app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

    let last_module =
        app.init_modules(|router, _, store| router.custom.get_last_module(store).unwrap());

    assert_eq!(last_module, None);

    // Update the ubtc and ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65 prices to match the order rate.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t1))
        .unwrap();

    // Execute the order processing.
    app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

    let last_module = app
        .init_modules(|router, _, store| router.custom.get_last_module(store).unwrap())
        .unwrap();

    assert_eq!(last_module, "PerpetualClose");
}
