use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Int128, Uint128};

#[cw_serde]
#[derive(Default)]
pub struct AprEdenBoost {
    pub uusdc: Uint128,
    pub ueden: Uint128,
}
#[cw_serde]
pub struct AprUsdc {
    pub uusdc: Int128,
    pub ueden: Int128,
}

// implement default
impl Default for AprUsdc {
    fn default() -> Self {
        Self {
            uusdc: Int128::zero(),
            ueden: Int128::zero(),
        }
    }
}

#[cw_serde]
pub struct AprElys {
    pub uusdc: Uint128,
    pub ueden: Uint128,
    pub uedenb: Uint128,
}

// implement default
impl Default for AprElys {
    fn default() -> Self {
        Self {
            uusdc: Uint128::zero(),
            ueden: Uint128::zero(),
            uedenb: Uint128::zero(),
        }
    }
}

#[cw_serde]
pub struct BalanceBorrowed {
    pub usd_amount: Decimal,
    pub percentage: Decimal,
}

// implement default
impl Default for BalanceBorrowed {
    fn default() -> Self {
        Self {
            usd_amount: Decimal::zero(),
            percentage: Decimal::zero(),
        }
    }
}

#[cw_serde]
pub struct QueryAprResponse {
    pub apr: Uint128,
}

// implement default
impl Default for QueryAprResponse {
    fn default() -> Self {
        Self {
            apr: Uint128::zero(),
        }
    }
}

#[cw_serde]
pub struct BalanceReward {
    pub asset: String,
    pub amount: Uint128,
    pub usd_amount: Option<Decimal>,
}

// implement default
impl Default for BalanceReward {
    fn default() -> Self {
        Self {
            asset: "".to_string(),
            amount: Uint128::zero(),
            usd_amount: None,
        }
    }
}

#[cw_serde]
pub struct StakingValidator {
    // The validator identity.
    pub id: String,
    // The validator address.
    pub address: String,
    // The validator name.
    pub name: String,
    // Voting power percentage for this validator.
    pub voting_power: Decimal,
    // commission percentage for the validator.
    pub commission: Decimal
}

// implement default
impl Default for StakingValidator {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            address: "".to_string(),
            name: "".to_string(),
            voting_power: Decimal::zero(),
            commission: Decimal::zero()
        }
    }
}
