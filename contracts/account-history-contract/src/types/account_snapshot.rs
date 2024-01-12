use cosmwasm_schema::cw_serde;
use cosmwasm_std::DecCoin;
use cw_utils::Expiration;

use super::CoinValue;

#[cw_serde]
pub struct AccountSnapshot {
    pub date: Expiration,
    pub total_liquid_asset_balance: DecCoin,
    pub total_avaible_balance: DecCoin,
    pub total_in_orders_balance: DecCoin,
    pub avaible_asset_balance: Vec<CoinValue>,
    pub in_orders_asset_balance: Vec<CoinValue>,
}
