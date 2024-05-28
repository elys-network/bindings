use cw_storage_plus::Map;
use elys_bindings::account_history::types::PortfolioBalanceSnapshot;

pub const OLD_HISTORY_1: Map<&str, PortfolioBalanceSnapshot> =
    Map::new("history_portfolio_balance_snapshot_1");

pub const OLD_HISTORY_2: Map<&str, PortfolioBalanceSnapshot> =
    Map::new("history_portfolio_balance_snapshot_2");

pub const HISTORY: Map<&str, PortfolioBalanceSnapshot> =
    Map::new("history_portfolio_balance_snapshot_3");
