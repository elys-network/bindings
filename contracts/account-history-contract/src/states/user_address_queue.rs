use cw_storage_plus::Map;

pub const USER_ADDRESS_QUEUE: Map<&str, ()> = Map::new("user address queue");
