use cosmwasm_std::StdError;

use super::*;

#[test]
fn stake_request_error() {
    let mut app = ElysApp::new();

    let instantiate_msg = InstantiateMockMsg {
        account_history_address: None,
        spot_orders: vec![],
        perpetual_orders: vec![],
    };

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

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

    let msg = ExecuteMsg::StakeRequest {
        amount: 0,
        asset: "uusdc".to_string(),
        validator_address: None,
    };

    let err = app
        .execute_contract(Addr::unchecked("user"), addr.clone(), &msg, &[])
        .unwrap_err();

    assert_eq!(
        ContractError::StdError(StdError::generic_err("amount is zero")),
        err.downcast().unwrap()
    );

    let msg = ExecuteMsg::StakeRequest {
        amount: 100000000,
        asset: "uusdc".to_string(),
        validator_address: None,
    };

    let err = app
        .execute_contract(Addr::unchecked("user"), addr.clone(), &msg, &[])
        .unwrap_err();

    assert_eq!(
        ContractError::InsufficientBalanceError {
            balance: 100,
            amount: 100000000
        },
        err.downcast().unwrap()
    );

    let msg = ExecuteMsg::StakeRequest {
        amount: 10,
        asset: "uelys".to_string(),
        validator_address: None,
    };

    let err = app
        .execute_contract(Addr::unchecked("user"), addr.clone(), &msg, &[])
        .unwrap_err();

    assert_eq!(
        ContractError::StdError(StdError::generic_err(
            "The validator Address is required only if the staked asset is uelys"
        )),
        err.downcast().unwrap()
    );

    let msg = ExecuteMsg::StakeRequest {
        amount: 10,
        asset: "uusdc".to_string(),
        validator_address: None,
    };

    let resp = app.execute_contract(Addr::unchecked("user"), addr.clone(), &msg, &[]);

    assert!(resp.is_ok());
}
