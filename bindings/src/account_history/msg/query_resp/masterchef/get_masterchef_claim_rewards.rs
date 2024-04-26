use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Default)]
pub struct GetMasterchefClaimRewardsResponse {
    pub code: u64,
    pub result: String,
}
