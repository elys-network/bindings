use cw_storage_plus::Item;

pub const PARAMS_ADMIN: Item<String> = Item::new("params admin");

pub const MARKET_ORDER_ENABLED: Item<bool> = Item::new("market_order");
pub const STAKE_ENABLED: Item<bool> = Item::new("stake_endpoint");
pub const PROCESS_ORDERS_ENABLED: Item<bool> = Item::new("process_orders_enabled");
pub const SWAP_ENABLED: Item<bool> = Item::new("swap_enabled");
pub const PERPETUAL_ENABLED: Item<bool> = Item::new("perpetual_enabled");
pub const REWARD_ENABLED: Item<bool> = Item::new("reward_enable");
pub const LEVERAGE_ENABLED: Item<bool> = Item::new("leverage_enable");
