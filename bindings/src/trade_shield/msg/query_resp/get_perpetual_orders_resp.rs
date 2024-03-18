use crate::{trade_shield::types::PerpetualOrderPlus, types::PageResponse};
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetPerpetualOrdersResp {
    pub page_response: Option<PageResponse>,
    pub orders: Vec<PerpetualOrderPlus>,
}

impl GetPerpetualOrdersResp {
    pub fn empty(have_total: bool) -> Self {
        Self {
            page_response: Some(PageResponse::empty(have_total)),
            orders: vec![],
        }
    }
}
