use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

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
