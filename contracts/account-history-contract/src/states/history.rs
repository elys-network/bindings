use cw_storage_plus::{Item, Map};
use elys_bindings::account_history::types::PortfolioBalanceSnapshot;

pub const OLD_HISTORY_1: Map<&str, PortfolioBalanceSnapshot> =
    Map::new("history_portfolio_balance_snapshot_1");

pub const OLD_HISTORY_2: Map<&str, PortfolioBalanceSnapshot> =
    Map::new("history_portfolio_balance_snapshot_2");

pub const HISTORY: Map<&str, PortfolioBalanceSnapshot> =
    Map::new("history_portfolio_balance_snapshot_3");

pub const DELETE_OLD_DATA_ENABLED: Item<bool> = Item::new("delete_old_data_enabled");
