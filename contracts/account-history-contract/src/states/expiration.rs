use cw_storage_plus::Item;
use cw_utils::Expiration;

pub const EXPIRATION: Item<Expiration> = Item::new("expiration");
