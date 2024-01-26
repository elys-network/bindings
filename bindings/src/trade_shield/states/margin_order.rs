use crate::trade_shield::types::MarginOrder;
use cw_storage_plus::Map;

pub const MARGIN_ORDER: Map<u64, MarginOrder> = Map::new("margin order2");

pub const PENDING_MARGIN_ORDER: Map<u64, MarginOrder> = Map::new("unprocess margin order");
