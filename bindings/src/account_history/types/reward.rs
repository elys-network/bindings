use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal256;

#[cw_serde]
pub struct Reward {
    pub usdc_usd: Decimal256,
    pub eden_usd: Decimal256,
    pub eden_boost: Decimal256,
    pub other_usd: Decimal256,
    pub total_usd: Decimal256,
}

// implement default
impl Default for Reward {
    fn default() -> Self {
        Self {
            usdc_usd: Decimal256::zero(),
            eden_usd: Decimal256::zero(),
            eden_boost: Decimal256::zero(),
            other_usd: Decimal256::zero(),
            total_usd: Decimal256::zero(),
        }
    }
}
