use crate::types::Portfolio;
use cw_storage_plus::Map;

pub const PORTFOLIO: Map<&str, Portfolio> = Map::new("portfolio");
