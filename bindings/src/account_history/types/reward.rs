use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct Reward {
    pub usdc_usd: Decimal,
    pub eden_usd: Decimal,
    pub eden_boost: Decimal,
    pub other_usd: Decimal,
    pub total_usd: Decimal,
}

// implement default
impl Default for Reward {
    fn default() -> Self {
        Self {
            usdc_usd: Decimal::zero(),
            eden_usd: Decimal::zero(),
            eden_boost: Decimal::zero(),
            other_usd: Decimal::zero(),
            total_usd: Decimal::zero(),
        }
    }
}
