use cosmwasm_std::StdError;

use super::*;

#[test]
fn elys_cancel_unstake_request_error() {
    let mut app = ElysApp::new();

    let instantiate_msg = InstantiateMockMsg {
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

    let msg = ExecuteMsg::ElysCancelUnstakeRequest {
        amount: coin(0, "usdc"),
        validator_address: "validator".to_string(),
        creation_height: 50,
    };

    let err = app
        .execute_contract(Addr::unchecked("user"), addr.clone(), &msg, &[])
        .unwrap_err();

    assert_eq!(
        ContractError::StdError(StdError::generic_err("amount is zero")),
        err.downcast().unwrap()
    );

    let msg = ExecuteMsg::ElysCancelUnstakeRequest {
        amount: coin(10, "usdc"),
        validator_address: "validator".to_string(),
        creation_height: 51,
    };

    let resp = app.execute_contract(Addr::unchecked("user"), addr.clone(), &msg, &[]);

    assert!(resp.is_ok());
}
