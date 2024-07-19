use cosmwasm_std::StdError;

use super::*;

#[test]
fn elys_redelegation_request_error() {
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

    let msg = ExecuteMsg::ElysRedelegateRequest {
        amount: coin(0, "uusdc"),

        validator_src_address: "".to_string(),
        validator_dst_address: "".to_string(),
    };

    let err = app
        .execute_contract(Addr::unchecked("user"), addr.clone(), &msg, &[])
        .unwrap_err();

    assert_eq!(
        ContractError::StdError(StdError::generic_err("amount is zero")),
        err.downcast().unwrap()
    );

    let msg = ExecuteMsg::ElysRedelegateRequest {
        amount: coin(10, "uelys"),
        validator_src_address: "".to_string(),
        validator_dst_address: "validator".to_string(),
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

    let msg = ExecuteMsg::ElysRedelegateRequest {
        amount: coin(10, "uelys"),
        validator_src_address: "validator".to_string(),
        validator_dst_address: "".to_string(),
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

    let msg = ExecuteMsg::ElysRedelegateRequest {
        amount: coin(10, "uelys"),
        validator_src_address: "validator_src".to_string(),
        validator_dst_address: "validator_dst".to_string(),
    };

    let resp = app.execute_contract(Addr::unchecked("user"), addr.clone(), &msg, &[]);

    assert!(resp.is_ok());
}
