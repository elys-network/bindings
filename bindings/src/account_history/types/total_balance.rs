use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal256;

#[cw_serde]
pub struct TotalBalance {
    pub total_balance: Decimal256,
    pub portfolio_usd: Decimal256,
    pub reward_usd: Decimal256,
}

// implement zero
impl TotalBalance {
    pub fn zero(_value_denom: &String) -> Self {
        Self {
            total_balance: Decimal256::zero(),
            portfolio_usd: Decimal256::zero(),
            reward_usd: Decimal256::zero(),
        }
    }
}

// implement default
impl Default for TotalBalance {
    fn default() -> Self {
        Self::zero(&"usdc".to_string())
    }
}
