use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct TradeShieldParamsResponse {
    pub params_admis: String,
    pub market_order_enabled: bool,
    pub stake_enabled: bool,
    pub process_order_enabled: bool,
    pub swap_enabled: bool,
    pub perpetual_enabled: bool,
    pub reward_enabled: bool,
    pub leverage_enabled: bool,
}
