use crate::types::Portfolio;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetPortfolioResp {
    pub portfolio: Portfolio,
}
