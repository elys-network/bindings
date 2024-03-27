use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct NumberOfPendingOrderResp {
    pub spot_orders: u128,
    pub perpetual_orders: u128,
}
