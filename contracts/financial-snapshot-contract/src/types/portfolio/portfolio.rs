use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Portfolio {
    pub balance_usd: u64,
    pub liquid_assets_usd: u64,
    pub staked_committed_usd: u64,
    pub liquidity_positions_usd: u64,
    pub leverage_lp_usd: u64,
    pub margin_usd: u64,
    pub usdc_earn_usd: u64,
    pub borrows_usd: u64,
}
