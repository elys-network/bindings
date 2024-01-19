use std::fmt;
use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal256;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Tier fee discount is
// Bronze tier: standard ( no discount)
// Silver tier: > $50k balance ( 10% discount)
// Gold tier: > $250K balance ( 20% discount)
// Platinum tier: > $500K balance ( 30% discount)

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum MembershipTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl FromStr for MembershipTier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bronze" => Ok(MembershipTier::Bronze),
            "silver" => Ok(MembershipTier::Silver),
            "gold" => Ok(MembershipTier::Gold),
            "platinum" => Ok(MembershipTier::Platinum),
            _ => Err(()),
        }
    }
}

impl fmt::Display for MembershipTier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MembershipTier::Bronze => "bronze",
                MembershipTier::Silver => "silver",
                MembershipTier::Gold => "gold",
                MembershipTier::Platinum => "platinum",
            }
        )
    }
}

#[cw_serde]
pub struct MembershipTierResponse {
    pub identifier: MembershipTier,
    pub name: String,
}

impl MembershipTierResponse {
    pub fn zero() -> Self {
        Self::calc(Decimal256::zero())
    }

    pub fn calc(balance: Decimal256) -> Self {
        if balance > Decimal256::from_str("500000").unwrap() {
            Self {
                identifier: MembershipTier::Platinum,
                name: "Platinum".to_string(),
            }
        } else if balance > Decimal256::from_str("250000").unwrap() {
            Self {
                identifier: MembershipTier::Gold,
                name: "Gold".to_string(),
            }
        } else if balance > Decimal256::from_str("50000").unwrap() {
            Self {
                identifier: MembershipTier::Silver,
                name: "Silver".to_string(),
            }
        } else {
            Self {
                identifier: MembershipTier::Bronze,
                name: "Bronze".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            MembershipTier::from_str("bronze").unwrap(),
            MembershipTier::Bronze
        );
        assert_eq!(
            MembershipTier::from_str("silver").unwrap(),
            MembershipTier::Silver
        );
        assert_eq!(
            MembershipTier::from_str("gold").unwrap(),
            MembershipTier::Gold
        );
        assert_eq!(
            MembershipTier::from_str("platinum").unwrap(),
            MembershipTier::Platinum
        );
        assert!(MembershipTier::from_str("invalid").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", MembershipTier::Bronze), "bronze");
        assert_eq!(format!("{}", MembershipTier::Silver), "silver");
        assert_eq!(format!("{}", MembershipTier::Gold), "gold");
        assert_eq!(format!("{}", MembershipTier::Platinum), "platinum");
    }

    #[test]
    fn test_zero() {
        let response = MembershipTierResponse::zero();
        assert_eq!(response.name, "Bronze");
        assert_eq!(response.identifier, MembershipTier::Bronze);
    }

    #[test]
    fn test_calc() {
        let response = MembershipTierResponse::calc(Decimal256::from_str("10000").unwrap());
        assert_eq!(response.name, "Bronze");
        assert_eq!(response.identifier, MembershipTier::Bronze);

        let response = MembershipTierResponse::calc(Decimal256::from_str("60000").unwrap());
        assert_eq!(response.name, "Silver");
        assert_eq!(response.identifier, MembershipTier::Silver);

        let response = MembershipTierResponse::calc(Decimal256::from_str("300000").unwrap());
        assert_eq!(response.name, "Gold");
        assert_eq!(response.identifier, MembershipTier::Gold);

        let response = MembershipTierResponse::calc(Decimal256::from_str("600000").unwrap());
        assert_eq!(response.name, "Platinum");
        assert_eq!(response.identifier, MembershipTier::Platinum);
    }
}
