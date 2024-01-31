use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum PerpetualOrderType {
    LimitOpen,
    LimitClose,

    MarketOpen,
    MarketClose,

    StopLoss,
}
