use crate::msg::query_resp::GetMarginOrderResp;
use cosmwasm_std::{Addr, Decimal, SignedDecimal, SignedDecimal256};

use super::*;
// This test case verifies the successful query of an existing order in the contract.
#[test]
fn successful_query_message() {
    // Initialize the ElysApp instance.
    let mut app = ElysApp::new();

    let order = MarginOrder::new_open(
        "user",
        &MarginPosition::Long,
        &MarginOrderType::MarketOpen,
        &coin(255, "usdc"),
        "btc",
        &SignedDecimal::one(),
        &SignedDecimal256::one(),
        &Some(OrderPrice {
            base_denom: "btc".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Decimal::one(),
        }),
        &vec![],
    )
    .unwrap();

    // Create a mock message to instantiate the contract with an initial dummy order.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        margin_orders: vec![order.clone()],
    };

    // Extract the order ID from the dummy order.
    let id = instantiate_msg.margin_orders[0].order_id;

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

    // Query the contract for the existing order.
    let resp: GetMarginOrderResp = app
        .wrap()
        .query_wasm_smart(&addr, &QueryMsg::GetMarginOrder { id })
        .unwrap();

    // Verify that the response matches the expected order (the initial dummy order).
    assert_eq!(resp, GetMarginOrderResp { order });
}
