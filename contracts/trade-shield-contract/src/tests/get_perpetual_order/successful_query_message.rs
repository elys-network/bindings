use crate::msg::query_resp::GetPerpetualOrderResp;
use cosmwasm_std::{Addr, SignedDecimal, SignedDecimal256};

use super::*;
// This test case verifies the successful query of an existing order in the contract.
#[test]
fn successful_query_message() {
    // Initialize the ElysApp instance.
    let mut app = ElysApp::new();

    let order = PerpetualOrder {
        order_id: 1,
        owner: "user".to_string(),
        order_type: PerpetualOrderType::MarketOpen,
        position: PerpetualPosition::Long,
        trigger_price: Some(OrderPrice {
            base_denom: "btc".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Decimal::from_str("35.0").unwrap(),
        }),
        collateral: coin(255, "usdc"),
        trading_asset: "btc".to_string(),
        leverage: SignedDecimal::from_str("5").unwrap(),
        take_profit_price: Some(SignedDecimal256::one()),
        position_id: None,
        status: Status::Executed,
    };

    // Create a mock message to instantiate the contract with an initial dummy order.
    let instantiate_msg = InstantiateMockMsg {
        account_history_address: None,
        spot_orders: vec![],
        perpetual_orders: vec![order.clone()],
    };

    // Extract the order ID from the dummy order.
    let id = instantiate_msg.perpetual_orders[0].order_id;

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
    let resp: GetPerpetualOrderResp = app
        .wrap()
        .query_wasm_smart(&addr, &QueryMsg::GetPerpetualOrder { id })
        .unwrap();

    // custody = collateral * leverage / trigger_price
    // custody = 255 * 5 / 35 = 36...
    let custody = coin(36, &order.trading_asset);

    // Verify that the response matches the expected order (the initial dummy order).
    assert_eq!(resp.order.order, order);
    assert_eq!(resp.order.custody, custody);
}
