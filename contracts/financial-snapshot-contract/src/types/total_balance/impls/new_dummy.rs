use crate::types::{total_balance::total_balance::TotalBalance};

impl TotalBalance {
    pub fn new_dummy() -> TotalBalance {
        TotalBalance {
            total_balance: 100,
            portfolio_usd: 50,
            reward_usd: 50,
        }
    }
}
