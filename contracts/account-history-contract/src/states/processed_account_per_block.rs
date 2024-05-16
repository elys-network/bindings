use cw_storage_plus::Item;

pub const PROCESSED_ACCOUNT_PER_BLOCK: Item<u64> = Item::new("processed account per block");
