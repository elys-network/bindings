use cosmwasm_std::Timestamp;

use super::*;

#[test]
fn unauthorize() {
    let wallets = vec![("owner", coins(10_000000, "uelys"))];
    let mut app = ElysApp::new_with_wallets(wallets);

    // Create a mock message to instantiate the contract with an empty list of orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![SpotOrder {
            order_type: SpotOrderType::LimitBuy,
            order_id: 1,
            order_price: OrderPrice {
                base_denom: "uusdc".to_string(),
                quote_denom: "uelys".to_string(),
                rate: Decimal::from_str("0.258478").unwrap(),
            },
            order_amount: coin(10_000000, "uelys"),
            owner_address: Addr::unchecked("user"),
            order_target_denom: "uusdc".to_string(),
            status: Status::Pending,
            date: Date {
                height: 5,
                time: Timestamp::from_seconds(15),
            },
        }],
        perpetual_orders: vec![],
    };

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(10_000000, "uelys"),
            "Contract",
            None,
        )
        .unwrap();

    let sender = Addr::unchecked("not_user");

    let err = app
        .execute_contract(
            sender.clone(),
            addr,
            &&ExecuteMsg::CancelSpotOrders {
                order_ids: Some(vec![1]),
                order_type: None,
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::Unauthorized { sender },
        err.downcast().unwrap()
    );
}
