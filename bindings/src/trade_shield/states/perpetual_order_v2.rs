use crate::trade_shield::types::PerpetualOrderV2;
use cw_storage_plus::Map;

pub const PERPETUAL_ORDER_V2: Map<u64, PerpetualOrderV2> = Map::new("perpetual order2_v2");

pub const PENDING_PERPETUAL_ORDER_V2: Map<u64, PerpetualOrderV2> =
    Map::new("unprocess perpetual order_v2");
