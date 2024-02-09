use crate::account_history::types::earn_program::elys_earn::ElysEarnProgram;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetElysEarnProgramResp {
    pub data: ElysEarnProgram,
}
