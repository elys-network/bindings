use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct AprUsdc {
    pub uusdc: Uint128,
    pub ueden: Uint128,
}

#[cw_serde]
pub struct AprElys {
    pub uusdc: Uint128,
    pub ueden: Uint128,
    pub uedenb: Uint128,
}

#[cw_serde]
pub struct QueryAprResponse {
    pub apr: Uint128,
}

#[cw_serde]
pub struct BalanceReward {
    pub asset: String,
    pub amount: Uint128,
    pub usd_amount: Option<Decimal>,
}
