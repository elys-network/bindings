use cosmwasm_schema::cw_serde;

use crate::account_history::types::earn_program::eden_earn::EdenEarnProgram;

#[cw_serde]
#[derive(Default)]
pub struct GetEdenEarnProgramResp {
    pub data: EdenEarnProgram,
}
