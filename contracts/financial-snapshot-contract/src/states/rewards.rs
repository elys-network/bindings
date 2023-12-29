use crate::types::Reward;
use cw_storage_plus::Map;

pub const REWARDS: Map<&str, Reward> = Map::new("rewards");
