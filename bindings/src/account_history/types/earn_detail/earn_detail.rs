use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct AprUsdc {
    pub uusdc: Uint128,
    pub ueden: Uint128,
}

// implement default
impl Default for AprUsdc {
    fn default() -> Self {
        Self {
            uusdc: Uint128::zero(),
            ueden: Uint128::zero(),
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
    // The validator address.
    pub address: String,
    // The validator name.
    pub name: String,
    // Voting power percentage for this validator.
    pub voting_power: Decimal,
    // commission percentage for the validator.
    pub commission: Decimal,
    // The url of the validator profile picture
    pub profile_picture_src: Option<String>,
}

// implement default
impl Default for StakingValidator {
    fn default() -> Self {
        Self {
            address: "".to_string(),
            name: "".to_string(),
            voting_power: Decimal::zero(),
            commission: Decimal::zero(),
            profile_picture_src: None,
        }
    }
}
