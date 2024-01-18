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
pub struct BalanceBorrowed {
    pub usd_amount: Decimal,
    pub percentage: Decimal,
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
