use cosmwasm_schema::cw_serde;
use cosmwasm_std::DecCoin;

use super::CoinValue;

#[cw_serde]
pub struct LiquidAsset {
    pub total_liquid_asset_balance: DecCoin,
    pub total_available_balance: DecCoin,
    pub total_in_orders_balance: DecCoin,
    pub available_asset_balance: Vec<CoinValue>,
    pub in_orders_asset_balance: Vec<CoinValue>,
    pub total_value_per_asset: Vec<CoinValue>,
}
