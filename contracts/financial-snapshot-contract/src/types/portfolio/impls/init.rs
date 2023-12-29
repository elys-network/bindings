use crate::types::portfolio::portfolio::Portfolio;

impl Portfolio {
    pub fn init() -> Portfolio {
        Portfolio {
            balance_usd: 0,
            liquid_assets_usd: 0,
            staked_committed_usd: 0,
            liquidity_positions_usd: 0,
            leverage_lp_usd: 0,
            margin_usd: 0,
            usdc_earn_usd: 0,
            borrows_usd: 0,
        }
    }
}
