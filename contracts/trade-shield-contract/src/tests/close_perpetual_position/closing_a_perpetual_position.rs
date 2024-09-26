use cosmwasm_std::{Int128, SignedDecimal, SignedDecimal256};
use elys_bindings::{query_resp::PerpetualGetPositionsForAddressResponse, ElysQuery};

use super::*;

#[test]
fn closing_perpetualg_position() {
    // Initialize the ElysApp.
    let mut app = ElysApp::new();
    let mtps = vec![Mtp {
        address: "user".to_string(),
        amm_pool_id: 1,
        borrow_interest_paid_collateral: Int128::zero(),
        borrow_interest_paid_custody: Int128::zero(),
        borrow_interest_unpaid_collateral: Int128::zero(),
        collateral_asset: "uusdc".to_string(),
        collateral: Int128::new(1000000),
        consolidate_leverage: SignedDecimal::zero(),
        custody: Int128::new(5000000),
        custody_asset: "uusdc".to_string(),
        funding_fee_paid_collateral: Int128::zero(),
        funding_fee_paid_custody: Int128::zero(),
        funding_fee_received_collateral: Int128::zero(),
        funding_fee_received_custody: Int128::zero(),
        id: 2,
        leverage: SignedDecimal::from_str("5.0").unwrap(),
        liabilities: Int128::zero(),
        liabilities_asset: "uusdc".to_string(),
        mtp_health: SignedDecimal::one(),
        open_price: SignedDecimal::zero(),
        position: 1,
        sum_collateral: Int128::zero(),
        take_profit_borrow_rate: SignedDecimal::zero(),
        take_profit_custody: Int128::zero(),
        take_profit_liabilities: Int128::zero(),
        take_profit_price: SignedDecimal256::from_str("30").unwrap(),
        trading_asset: "uatom".to_string(),
        stop_loss_price: SignedDecimal::zero(),
        last_interest_calc_time: None,
        last_interest_calc_block: None,
        last_funding_calc_time: None,
        last_funding_calc_block: None,
    }];

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        perpetual_orders: vec![],
    };

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
    let code_id = app.store_code(Box::new(code));

    app.init_modules(|router, _, store| router.custom.set_mtp(store, &mtps))
        .unwrap();

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

    app.execute_contract(
        Addr::unchecked("user"),
        addr,
        &ExecuteMsg::ClosePerpetualPosition {
            id: 2,
            amount: Int128::new(5000000),
        },
        &[],
    )
    .unwrap();

    let r: PerpetualGetPositionsForAddressResponse = app
        .wrap()
        .query(&ElysQuery::perpetual_get_position_for_address("user".to_string(), None).into())
        .unwrap();

    assert!(r.mtps.is_empty());

    let last_module_used = app
        .init_modules(|router, _, storage| router.custom.get_last_module(storage).unwrap())
        .unwrap();
    assert_eq!(last_module_used, "PerpetualClose");
}
