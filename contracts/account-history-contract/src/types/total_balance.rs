use cosmwasm_schema::cw_serde;
use cosmwasm_std::DecCoin;

#[cw_serde]
pub struct TotalBalance {
    pub total_balance: DecCoin,
    pub portfolio_usd: DecCoin,
    pub reward_usd: DecCoin,
}
