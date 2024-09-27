use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

use crate::query_resp::CoinNeg;

#[cw_serde]
pub struct Fee {
    pub percent: String,
    pub amount: Coin,
}

impl Default for Fee {
    fn default() -> Self {
        Self {
            percent: "".to_string(),
            amount: Coin::default(),
        }
    }
}

#[cw_serde]
pub struct FeeNeg {
    pub percent: String,
    pub amount: CoinNeg,
}

impl Default for FeeNeg {
    fn default() -> Self {
        Self {
            percent: "".to_string(),
            amount: CoinNeg::default(),
        }
    }
}
