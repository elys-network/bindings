use crate::types::Portfolio;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::SignedDecimal256;

#[cw_serde]
pub struct GetPortfolioResp {
    pub portfolio: Portfolio,
    pub price_difference: SignedDecimal256,
}
