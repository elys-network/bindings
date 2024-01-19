use crate::types::AccountSnapshot;
use cw_storage_plus::Map;

pub const HISTORY: Map<&str, Vec<AccountSnapshot>> = Map::new("history4");
