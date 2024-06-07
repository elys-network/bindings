use cosmwasm_std::{Int128, SignedDecimal, StdError};

use super::*;

#[test]
fn invalid_leverage() {
    let mut app = ElysApp::new();

    let trade_shield_code = ContractWrapper::new(execute, instantiate, query);
    let trade_shield_code_id = app.store_code(Box::new(trade_shield_code));
    let trade_shield_init = InstantiateMockMsg {
        account_history_address: None,
        spot_orders: vec![],
        perpetual_orders: vec![],
    };

    let addr = app
        .instantiate_contract(
            trade_shield_code_id,
            Addr::unchecked("admin"),
            &trade_shield_init,
            &[],
            "contract",
            None,
        )
        .unwrap();

    let invalid_message = ExecuteMsg::LeveragelpOpen {
        amm_pool_id: 1,
        collateral_asset: "uusdc".to_string(),
        collateral_amount: Int128::new(1000000),
        leverage: SignedDecimal::one(),
        stop_loss_price: SignedDecimal::zero(),
    };

    let valid_message = ExecuteMsg::LeveragelpOpen {
        amm_pool_id: 1,
        collateral_asset: "uusdc".to_string(),
        collateral_amount: Int128::new(1000000),
        leverage: SignedDecimal::from_str("2.0").unwrap(),
        stop_loss_price: SignedDecimal::zero(),
    };

    let error = app
        .execute_contract(Addr::unchecked("user"), addr.clone(), &invalid_message, &[])
        .unwrap_err();

    assert_eq!(
        ContractError::StdError(StdError::generic_err("leverage must be greater than 1")),
        error.downcast().unwrap()
    );

    app.execute_contract(Addr::unchecked("user"), addr, &valid_message, &[])
        .unwrap();
}
