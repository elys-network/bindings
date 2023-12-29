use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct LiquidAsset {
    pub asset: String,
    pub change_percent_24hr: Decimal,
    pub total_usd: Decimal,
    pub total_token: u64,
    pub available_usd: Decimal,
    pub available_token: u64,
    pub in_order_usd: Decimal,
    pub in_order_token: u64,
}
