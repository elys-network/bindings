use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct TotalBalance {
    pub total_balance: u64,
    pub portfolio_usd: u64,
    pub reward_usd: u64,
}
