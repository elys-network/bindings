use cw_storage_plus::Item;
use cw_utils::Duration;

pub const INTERVAL: Item<Duration> = Item::new("interval");
