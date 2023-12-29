use crate::types::portfolio::portfolio::Portfolio;

impl Portfolio {
    pub fn new_dummy() -> Portfolio {
        Portfolio {
            balance_usd: 100,
            liquid_assets_usd: 100,
            staked_committed_usd: 10,
            liquidity_positions_usd: 10,
            leverage_lp_usd: 0,
            margin_usd: 0,
            usdc_earn_usd: 0,
            borrows_usd: 0,
        }
    }
}
