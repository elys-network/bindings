use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;
use elys_bindings::types::EarnType;

#[cw_serde]
pub struct StakedAsset {
    pub program: EarnType,
    pub bonding_period: u64,
    pub apr: Decimal,
    pub available: Decimal,
    pub staked: Decimal,
    pub rewards: Decimal,
}
