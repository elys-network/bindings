use cw_storage_plus::Item;
use elys_bindings::account_history::types::Metadata;

pub const METADATA: Item<Metadata> = Item::new("metadata");
