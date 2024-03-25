use cw_storage_plus::Item;

pub const LIMIT_PROCESS_ORDER: Item<Option<u128>> = Item::new("limit order processed");
