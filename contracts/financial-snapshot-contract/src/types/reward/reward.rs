use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct Reward {
    pub unclaimed_usdc_usd: Decimal,
    pub unclaimed_eden_usd: Decimal,
    pub unclaimed_eden_boost: u64,
    pub external_rewards_usd: Decimal,
}
