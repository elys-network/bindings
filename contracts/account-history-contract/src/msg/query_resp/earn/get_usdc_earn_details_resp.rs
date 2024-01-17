use crate::types::earn_program::usdc_earn::UsdcEarnProgram;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetUsdcEarnProgramResp {
    pub data: UsdcEarnProgram,
}
