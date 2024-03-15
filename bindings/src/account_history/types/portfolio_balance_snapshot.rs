use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal256};
use cw_utils::Expiration;

#[cw_serde]
pub struct PortfolioBalanceSnapshot {
    pub date: Expiration,
    pub portfolio_balance_usd: DecCoin,
    pub total_balance_usd: DecCoin,
}

impl PortfolioBalanceSnapshot {
    pub fn zero(value_denom: &String) -> Self {
        Self {
            date: Expiration::Never {},
            portfolio_balance_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
            total_balance_usd: DecCoin::new(Decimal256::zero(), value_denom.to_string()),
        }
    }
}

impl Default for PortfolioBalanceSnapshot {
    fn default() -> Self {
        Self::zero(&"usdc".to_string())
    }
}
