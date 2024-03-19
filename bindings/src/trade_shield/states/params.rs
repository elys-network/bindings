use cw_storage_plus::Item;

pub const MARKET_ORDER: Item<bool> = Item::new("market_order");
pub const STAKE_ENDPOINT: Item<bool> = Item::new("stake_endpoint");