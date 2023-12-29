use crate::types::{total_balance::total_balance::TotalBalance};

impl TotalBalance {
    pub fn init() -> TotalBalance {
        TotalBalance {
            total_balance: 0,
            portfolio_usd: 0,
            reward_usd: 0,
        }
    }
}
