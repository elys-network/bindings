use cw_storage_plus::Item;
use elys_bindings::types::PageRequest;

pub const PAGINATION: Item<PageRequest> = Item::new("pagination");
