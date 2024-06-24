use std::collections::HashMap;

use cosmwasm_std::Decimal;
use cw_storage_plus::Map;

pub const PRICE_CACHE: Map<u64, HashMap<String, (Decimal, u64)>> = Map::new("price cache");
