use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal256;

// Tier fee discount is
// Bronze tier: standard ( no discount)
// Silver tier: > $50k balance ( 10% discount)
// Gold tier: > $250K balance ( 20% discount)
// Platinum tier: > $500K balance ( 30% discount)

#[cw_serde]
pub struct MembershipTierResponse {
    pub identifier: String,
    pub name: String,
}

impl MembershipTierResponse {
    pub fn zero() -> Self {
        Self::calc(Decimal256::zero())
    }

    pub fn calc(balance: Decimal256) -> Self {
        if balance > Decimal256::from_str("500000").unwrap() {
            Self {
                identifier: "platinum".to_string(),
                name: "Platinum".to_string(),
            }
        } else if balance > Decimal256::from_str("250000").unwrap() {
            Self {
                identifier: "gold".to_string(),
                name: "Gold".to_string(),
            }
        } else if balance > Decimal256::from_str("50000").unwrap() {
            Self {
                identifier: "silver".to_string(),
                name: "Silver".to_string(),
            }
        } else {
            Self {
                identifier: "bronze".to_string(),
                name: "Bronze".to_string(),
            }
        }
    }
}
