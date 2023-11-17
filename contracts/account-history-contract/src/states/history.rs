use crate::types::AccountValue;
use cw_storage_plus::Map;

pub const HISTORY: Map<&str, Vec<AccountValue>> = Map::new("history");
