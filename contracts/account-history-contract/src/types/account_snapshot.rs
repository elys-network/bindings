use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Uint128};
use cw_utils::Expiration;

#[cw_serde]
pub struct Rewards {
    pub usdc: Decimal, // in usd dollar
    pub eden: Decimal, // in usd dollar
    pub edenb: Uint128, // eden boost doens't have price
    pub other: Decimal, // in usd dollar
}

#[cw_serde]
pub struct AccountSnapshot {
    pub date: Expiration,
    pub account_value: Coin,
    pub locked_value: Coin,
    pub account_assets: Vec<Coin>,
    pub locked_asset : Vec<Coin>,
    pub rewards: Rewards,
}