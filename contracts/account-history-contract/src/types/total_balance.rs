use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal256};

#[cw_serde]
pub struct TotalBalance {
    pub total_balance: DecCoin,
    pub portfolio_usd: DecCoin,
    pub reward_usd: DecCoin,
}

// implement zero
impl TotalBalance {
    pub fn zero(value_denom: &String) -> Self {
        Self {
            total_balance: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            portfolio_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            reward_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
        }
    }
}

// implement default
impl Default for TotalBalance {
    fn default() -> Self {
        Self::zero(&"usdc".to_string())
    }
}
