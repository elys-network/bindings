use crate::types::earn_program::eden_boost_earn::EdenBoostEarnProgram;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetEdenBoostEarnProgramResp {
    pub data: EdenBoostEarnProgram,
}
