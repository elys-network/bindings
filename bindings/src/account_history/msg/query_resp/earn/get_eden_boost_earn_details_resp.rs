use crate::account_history::types::earn_program::eden_boost_earn::EdenBoostEarnProgram;

use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Default)]
pub struct GetEdenBoostEarnProgramResp {
    pub data: EdenBoostEarnProgram,
}
