use crate::types::TotalBalance;
use cw_storage_plus::Map;

pub const TOTAL_BALANCE: Map<&str, TotalBalance> = Map::new("total_balance");

