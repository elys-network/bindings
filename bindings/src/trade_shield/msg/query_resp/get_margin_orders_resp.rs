use cosmwasm_schema::cw_serde;
use crate::types::PageResponse;

use crate::trade_shield::types::MarginOrder;

#[cw_serde]
pub struct GetMarginOrdersResp {
    pub page_response: Option<PageResponse>,
    pub orders: Vec<MarginOrder>,
}

impl GetMarginOrdersResp {
    pub fn empty(have_total: bool) -> Self {
        Self {
            page_response: Some(PageResponse::empty(have_total)),
            orders: vec![],
        }
    }
}
