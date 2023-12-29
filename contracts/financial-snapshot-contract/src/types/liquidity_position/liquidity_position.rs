use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct LiquidityPosition {
    pub pool_id: u64,
    pub pool_rate_percent: Vec<Rate>,
    pub apr_usd: Decimal,
    pub inflationary_eden_rewards: u64,
    pub external_rewards_apr_usd: Decimal,
    pub fee_apr_usd: Decimal,
    pub fees_usd: Decimal,
    pub current_tvl_usd: Decimal,
    pub balance_usd: Decimal,
    pub rewards_usd: Decimal,
}

#[cw_serde]
pub struct Rate {
    pub denom: String,
    pub percent: Decimal,
}