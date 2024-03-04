use std::collections::HashMap;

use cw_storage_plus::Map;
use elys_bindings::account_history::types::AccountSnapshot;

pub const HISTORY: Map<&str, HashMap<String, AccountSnapshot>> = Map::new("history14");
