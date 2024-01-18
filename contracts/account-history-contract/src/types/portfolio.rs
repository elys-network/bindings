use cosmwasm_schema::cw_serde;
use cosmwasm_std::DecCoin;

#[cw_serde]
pub struct Portfolio {
    pub balance_usd: DecCoin,
    pub liquid_assets_usd: DecCoin,
    pub staked_committed_usd: DecCoin,
    pub liquidity_positions_usd: DecCoin,
    pub leverage_lp_usd: DecCoin,
    pub margin_usd: DecCoin,
    pub usdc_earn_usd: DecCoin,
    pub borrows_usd: DecCoin,
}
