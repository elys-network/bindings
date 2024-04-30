use cw_storage_plus::Item;

pub const NUMBER_OF_PENDING_ORDER: Item<u64> = Item::new("number_of_pending_order");
pub const NUMBER_OF_EXECUTED_ORDER: Item<u64> = Item::new("number_of_executed_order");
