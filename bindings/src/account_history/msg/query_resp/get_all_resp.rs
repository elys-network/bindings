use crate::account_history::types::AccountSnapshot;
use crate::types::PageResponse;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetAllResp {
    pub snapshot_list: Vec<(String, Vec<(String, AccountSnapshot)>)>,
    pub pagination: Option<PageResponse>,
}
