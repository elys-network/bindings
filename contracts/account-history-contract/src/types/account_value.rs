use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Uint128};
use cw_utils::Expiration;

#[cw_serde]
pub struct AccountValue {
    pub date: Expiration,
    pub elys_amount: Uint128,
    pub elys_value: Coin,
}
