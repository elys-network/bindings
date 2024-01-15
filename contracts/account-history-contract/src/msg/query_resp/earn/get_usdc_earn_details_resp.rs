use cosmwasm_schema::cw_serde;
use crate::types::earn_program::usdc_earn::UsdcEarnProgram;

#[cw_serde]
pub struct GetUsdcEarnProgramResp {
    pub data: UsdcEarnProgram,
}
