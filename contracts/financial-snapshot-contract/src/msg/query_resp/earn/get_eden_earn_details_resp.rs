use cosmwasm_schema::cw_serde;

use crate::types::earn_program::eden_earn::EdenEarnProgram;

#[cw_serde]
pub struct GetEdenEarnProgramResp {
    pub data: EdenEarnProgram,
}
