use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal256;
use cw_utils::Expiration;

#[cw_serde]
pub struct PortfolioBalanceSnapshot {
    pub date: Expiration,
    pub total_balance_usd: Decimal256,
}

impl PortfolioBalanceSnapshot {
    pub fn zero() -> Self {
        Self {
            date: Expiration::Never {},
            total_balance_usd: Decimal256::zero(),
        }
    }
}

impl Default for PortfolioBalanceSnapshot {
    fn default() -> Self {
        Self::zero()
    }
}

#[cw_serde]
pub struct PortfolioBalanceSnapshotOld {
    pub date: Expiration,
    pub portfolio_balance_usd: Decimal256,
    pub total_balance_usd: Decimal256,
}
