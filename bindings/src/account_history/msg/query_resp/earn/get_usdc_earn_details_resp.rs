use crate::account_history::types::earn_program::usdc_earn::UsdcEarnProgram;
use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Default)]
pub struct GetUsdcEarnProgramResp {
    pub data: UsdcEarnProgram,
}
