use elys_bindings::trade_shield::{
    msg::{
        query_resp::{GetSpotOrderResp, GetSpotOrderStatesResp},
        QueryMsg,
    },
    types::Status,
};
use elys_bindings_test::ElysApp;

pub fn test_spot_order_status(
    app: &ElysApp,
    contract_addr: String,
    order_id: u64,
    status: Status,
) -> () {
    let is_in_pending = status == Status::Pending;

    // Get Order State
    let order_states: GetSpotOrderStatesResp = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::GetSpotOrderStates { order_id },
        )
        .unwrap();

    // Get Order
    let GetSpotOrderResp { order }: GetSpotOrderResp = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetSpotOrder { order_id })
        .unwrap();

    assert_eq!(order_states.order, order);
    assert_eq!(order.status, status);
    assert_eq!(order_states.is_in_pending, is_in_pending);
    assert_eq!(order_states.is_in_pending_sorted_array, is_in_pending);
}
