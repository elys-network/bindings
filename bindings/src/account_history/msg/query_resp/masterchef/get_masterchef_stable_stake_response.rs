use cosmwasm_schema::cw_serde;
use cosmwasm_std::Int128;

#[cw_serde]
pub struct StableStakeAprResponse {
    pub apr: Int128,
}
