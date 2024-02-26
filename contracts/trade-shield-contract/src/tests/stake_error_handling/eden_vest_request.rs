use cosmwasm_std::StdError;

use super::*;

#[test]
fn eden_cancel_vest_request_error() {
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

    let msg = ExecuteMsg::EdenVestRequest { amount: 0 };

    let err = app
        .execute_contract(Addr::unchecked("user"), addr.clone(), &msg, &[])
        .unwrap_err();

    assert_eq!(
        ContractError::StdError(StdError::generic_err("amount is zero")),
        err.downcast().unwrap()
    );

    let msg = ExecuteMsg::EdenVestRequest { amount: 10 };

    let resp = app.execute_contract(Addr::unchecked("user"), addr.clone(), &msg, &[]);

    assert!(resp.is_ok());
}
