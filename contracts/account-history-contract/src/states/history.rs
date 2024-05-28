use cw_storage_plus::Map;
use elys_bindings::account_history::types::PortfolioBalanceSnapshot;

pub const HISTORY: Map<&str, PortfolioBalanceSnapshot> =
    Map::new("history_portfolio_balance_snapshot_3");
