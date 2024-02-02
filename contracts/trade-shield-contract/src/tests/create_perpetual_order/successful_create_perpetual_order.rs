use cosmwasm_std::{Addr, Decimal, Int64, SignedDecimal, SignedDecimal256, Uint128};
use std::str::FromStr;

use crate::tests::get_order_id_from_events::get_attr_from_events;

use super::*;

#[test]
fn successful_create_perpetual_order() {
    // Create a wallet for the "user" with an initial balance of 30000 ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65.
    let wallet = vec![(
        "user",
        coins(
            30000,
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
        ),
    )];

    // Initialize the ElysApp instance with the specified wallet.
    let mut app = ElysApp::new_with_wallets(wallet);

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

    // User "user" creates a non "MakerBuy" perpetual order for BTC
    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreatePerpetualOrder {
                position: Some(PerpetualPosition::Long),
                leverage: Some(SignedDecimal::from_atomics(Int64::new(215), 2).unwrap()),
                trading_asset: Some("btc".to_string()),
                take_profit_price: Some(
                    SignedDecimal256::from_atomics(Uint128::new(200), 2).unwrap(),
                ),
                order_type: PerpetualOrderType::LimitOpen,
                trigger_price: Some(OrderPrice {
                    base_denom:
                        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                            .to_string(),
                    quote_denom: "btc".to_string(),
                    rate: Decimal::from_str("40000.1").unwrap(),
                }),
                position_id: None,
            },
            &coins(
                30000,
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
            ), // User's BTC balance.
        )
        .unwrap();

    // Verify that the "user" no longer has any ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65 after creating the order.
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

    // Verify that the contract address locked the ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65.
    assert_eq!(
        app.wrap()
            .query_balance(
                &addr,
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            )
            .unwrap()
            .amount
            .u128(),
        30000
    );

    // Verify that an order ID is emitted in the contract's events.
    assert!(get_attr_from_events(&resp.events, "perpetual_order_id").is_some());
}
