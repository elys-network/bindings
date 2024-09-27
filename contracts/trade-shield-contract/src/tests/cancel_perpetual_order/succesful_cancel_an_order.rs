use super::*;
use cosmwasm_std::{Addr, DecCoin, Decimal, Decimal256, SignedDecimal, SignedDecimal256};

use cw_multi_test::BankSudo;

#[test]
fn succesful_cancel_an_order() {
    // Initialize the ElysApp.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        perpetual_orders: vec![PerpetualOrderV2::new_open(
            "user",
            &PerpetualPosition::Long,
            &PerpetualOrderType::LimitOpen,
            &coin(255, "usdc"),
            "btc",
            &SignedDecimal::from_str("1.2").unwrap(),
            &Some(SignedDecimal256::from_str("1.2").unwrap()),
            &Some(OrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_str("20000.0").unwrap(),
            }),
            &vec![],
            DecCoin::new(Decimal256::zero(), ""),
            SignedDecimal::zero(),
            Fee::default(),
            FeeNeg::default(),
        )
        .unwrap()],
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

    // Mint the token from the order to simulate that the tokens are already locked.

    app.sudo(
        BankSudo::Mint {
            to_address: addr.to_string(),
            amount: coins(255, "usdc"),
        }
        .into(),
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CancelPerpetualOrder { order_id: 0 },
        &[],
    )
    .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        255
    );
}
