use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct Reward {
    pub usdc_usd: Decimal,
    pub eden_usd: Decimal,
    pub eden_boost: Uint128,
    pub other_usd: Decimal,
    pub total_usd: Decimal,
}
