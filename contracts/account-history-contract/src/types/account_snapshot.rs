use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use cw_utils::Expiration;

#[cw_serde]
pub struct AccountSnapshot {
    pub date: Expiration,
    pub account_value: Coin,
    pub locked_value: Coin,
    pub account_assets: Vec<Coin>,
    pub locked_asset : Vec<Coin>,
}