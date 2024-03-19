use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct TradeShieldParamsResponse {
    pub market_order: bool,
    pub stake_request: bool
}