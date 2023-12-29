use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct GetUsdcPriceResp {
    pub price: Decimal,
}
