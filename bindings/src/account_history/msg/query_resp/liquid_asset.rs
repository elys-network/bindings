use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct LiquidAsset {
    pub denom: String,
    pub price: Decimal,
    pub available_amount: Decimal,
    pub available_value: Decimal,
    pub in_order_amount: Decimal,
    pub in_order_value: Decimal,
    pub total_amount: Decimal,
    pub total_value: Decimal,
}
