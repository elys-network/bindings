use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct TradeShieldParamsResponse {
    pub params_admis: String,
    pub market_order: bool,
    pub stake_endpoint: bool,
    pub process_order: bool,
    pub swap_endpoint: bool,
    pub perpetual_endpoint: bool,
    pub reward_endpoint: bool,
    pub leverage_endpoint: bool,
}
