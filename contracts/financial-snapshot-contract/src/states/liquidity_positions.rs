use crate::types::LiquidityPosition;
use cw_storage_plus::Item;

pub const LIQUIDITY_POSITIONS: Item<Vec<LiquidityPosition>> = Item::new("liquid_positions");
