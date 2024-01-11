use cosmwasm_schema::cw_serde;
use elys_bindings::types::PageResponse;

use crate::types::MarginOrder;

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
