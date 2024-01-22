use cosmwasm_schema::cw_serde;

use super::earn_program::{
    EdenBoostEarnProgram, EdenEarnProgram, ElysEarnProgram, UsdcEarnProgram,
};

#[cw_serde]
pub struct StakedAssets {
    pub eden_boost_earn_program: EdenBoostEarnProgram,
    pub eden_earn_program: EdenEarnProgram,
    pub elys_earn_program: ElysEarnProgram,
    pub usdc_earn_program: UsdcEarnProgram,
}

// implement default
impl Default for StakedAssets {
    fn default() -> Self {
        Self {
            eden_boost_earn_program: EdenBoostEarnProgram::default(),
            eden_earn_program: EdenEarnProgram::default(),
            elys_earn_program: ElysEarnProgram::default(),
            usdc_earn_program: UsdcEarnProgram::default(),
        }
    }
}
