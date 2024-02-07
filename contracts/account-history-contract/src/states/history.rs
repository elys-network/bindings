use std::collections::HashMap;

use crate::types::AccountSnapshot;
use cw_storage_plus::Map;

pub const HISTORY: Map<&str, HashMap<String, AccountSnapshot>> = Map::new("history11");
