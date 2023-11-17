use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Uint128};

#[cw_serde]
pub struct UserHistoryResponse {
    elys_amount: Uint128,
    elys_value: Coin,
}
