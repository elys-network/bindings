use crate::types::Price;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetAllPricesResponse {
    pub prices: Vec<Price>,
}
