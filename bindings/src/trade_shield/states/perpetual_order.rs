use crate::trade_shield::types::PerpetualOrder;
use cw_storage_plus::{Item, Map};

pub const PERPETUAL_ORDER: Map<u64, PerpetualOrder> = Map::new("perpetual order2");

pub const PENDING_PERPETUAL_ORDER: Map<u64, PerpetualOrder> = Map::new("unprocess perpetual order");

pub const USER_PERPETUAL_ORDER: Map<&str, Vec<u64>> = Map::new("user perpetual order");

pub const SORTED_PENDING_PERPETUAL_ORDER: Map<&str, Vec<u64>> = Map::new("sorted perpetual order");

pub const CLOSE_PERPETUAL_ORDER: Map<u64, Vec<u64>> = Map::new("close perpetual order");

pub const PERPETUAL_ORDER_MAX_ID: Item<u64> = Item::new("perpetual order max id");
