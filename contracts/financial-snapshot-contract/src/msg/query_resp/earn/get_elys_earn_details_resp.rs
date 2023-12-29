use cosmwasm_schema::cw_serde;
use crate::types::earn_program::elys_earn::ElysEarnProgram;

#[cw_serde]
pub struct GetElysEarnProgramResp {
    pub data: ElysEarnProgram,
}
