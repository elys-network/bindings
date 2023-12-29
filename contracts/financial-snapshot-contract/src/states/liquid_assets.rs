use crate::types::LiquidAsset;
use cw_storage_plus::Item;

pub const LIQUID_ASSETS: Item<Vec<LiquidAsset>> = Item::new("liquid_assets");
