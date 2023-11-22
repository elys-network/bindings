use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use cw_utils::Expiration;

#[cw_serde]
pub struct AccountValue {
    pub date: Expiration,
    pub account_value: Coin,
}
