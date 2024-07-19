use cosmwasm_std::{Addr, Decimal, Int128, SignedDecimal, SignedDecimal256};
use elys_bindings::trade_shield::msg::query_resp::GetPerpetualOrderResp;
use std::str::FromStr;

use crate::tests::get_order_id_from_events::get_attr_from_events;

use super::*;

#[test]
fn successful_create_perpetual_order() {
    // Initialize the ElysApp instance with the specified wallet.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        perpetual_orders: vec![],
    };

    let mtp_id = 2;

    let mtps = vec![Mtp {
        address: "user".to_owned(),
        amm_pool_id: 1,
        borrow_interest_paid_collateral: Int128::zero(),
        borrow_interest_paid_custody: Int128::zero(),
        borrow_interest_unpaid_collateral: Int128::zero(),
        collateral_asset: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            .to_owned(),
        collateral: Int128::new(100000000),
        consolidate_leverage: SignedDecimal::zero(),
        custody: Int128::new(100000000 * 5),
        custody_asset: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            .to_owned(),
        funding_fee_paid_collateral: Int128::zero(),
        funding_fee_paid_custody: Int128::zero(),
        funding_fee_received_collateral: Int128::zero(),
        funding_fee_received_custody: Int128::zero(),
        id: mtp_id,
        leverage: SignedDecimal::from_atomics(Int128::new(5), 0).unwrap(),
        liabilities: Int128::zero(),
        liabilities_asset: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            .to_owned(),
        mtp_health: SignedDecimal::one(),
        open_price: SignedDecimal::from_str("0.228252865828856914").unwrap(),
        position: PerpetualPosition::Short as i32,
        sum_collateral: Int128::zero(),
        take_profit_borrow_rate: SignedDecimal::zero(),
        take_profit_custody: Int128::zero(),
        take_profit_liabilities: Int128::zero(),
        take_profit_price: SignedDecimal256::from_str("0.28").unwrap(),
        trading_asset: "uelys".to_owned(),
    }];

    app.init_modules(|router, _, store| router.custom.set_mtp(store, &mtps))
        .unwrap();

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

    let first_trigger_price = Decimal::from_str("0.21").unwrap();
    let second_trigger_price = Decimal::from_str("0.20").unwrap();

    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreatePerpetualOrder {
                position: None,
                leverage: None,
                trading_asset: None,
                take_profit_price: None,
                order_type: PerpetualOrderType::StopLoss,
                trigger_price: Some(OrderPrice {
                    base_denom:
                        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                            .to_string(),
                    quote_denom: "uelys".to_string(),
                    rate: first_trigger_price.clone(),
                }),
                position_id: Some(mtp_id),
            },
            &[],
        )
        .unwrap();

    // Verify that an order ID is emitted in the contract's events.
    assert!(get_attr_from_events(&resp.events, "perpetual_order_id").is_some());

    let order_id: u64 = get_attr_from_events(&resp.events, "perpetual_order_id")
        .unwrap()
        .parse()
        .unwrap();

    let GetPerpetualOrderResp {
        order: PerpetualOrderPlus { order, .. },
    }: GetPerpetualOrderResp = app
        .wrap()
        .query_wasm_smart(addr.as_str(), &QueryMsg::GetPerpetualOrder { id: order_id })
        .unwrap();

    assert_eq!(order.trigger_price.unwrap().rate, first_trigger_price);

    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreatePerpetualOrder {
                position: None,
                leverage: None,
                trading_asset: None,
                take_profit_price: None,
                order_type: PerpetualOrderType::StopLoss,
                trigger_price: Some(OrderPrice {
                    base_denom:
                        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                            .to_string(),
                    quote_denom: "uelys".to_string(),
                    rate: second_trigger_price.clone(),
                }),
                position_id: Some(mtp_id),
            },
            &[],
        )
        .unwrap();

    assert!(get_attr_from_events(&resp.events, "perpetual_order_id").is_some());

    let same_order_id: u64 = get_attr_from_events(&resp.events, "perpetual_order_id")
        .unwrap()
        .parse()
        .unwrap();

    assert_eq!(same_order_id, order_id);

    let GetPerpetualOrderResp {
        order: PerpetualOrderPlus { order, .. },
    }: GetPerpetualOrderResp = app
        .wrap()
        .query_wasm_smart(addr.as_str(), &QueryMsg::GetPerpetualOrder { id: order_id })
        .unwrap();

    assert_eq!(order.trigger_price.unwrap().rate, second_trigger_price);
}
