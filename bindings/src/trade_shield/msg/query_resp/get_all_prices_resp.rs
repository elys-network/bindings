use cosmwasm_schema::cw_serde;
use crate::types::Price;

#[cw_serde]
pub struct GetAllPricesResponse {
    pub prices: Vec<Price>,
}
