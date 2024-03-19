use cw_storage_plus::Item;

pub const MARKET_ORDER_ENABLED: Item<bool> = Item::new("market_order");
pub const STAKE_ENABLED: Item<bool> = Item::new("stake_endpoint");
