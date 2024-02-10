use cw_storage_plus::Item;

pub const ACCOUNT_HISTORY_ADDRESS: Item<Option<String>> = Item::new("account_history_address");
