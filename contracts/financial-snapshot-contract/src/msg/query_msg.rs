#[allow(unused_imports)]
use super::query_resp::earn::*;
#[allow(unused_imports)]
use super::query_resp::pod::*;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw2::ContractVersion;
#[allow(unused_imports)]
use elys_bindings::query_resp::*;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Earn dashboard
    #[returns(QueryDelegatorValidatorsResponse)]
    GetAllValidators { delegator_addr: Option<String> },
    #[returns(QueryDelegatorValidatorsResponse)]
    GetDelegatorValidators { delegator_addr: String },
    #[returns(QueryDelegatorDelegationsResponse)]
    GetDelegations { delegator_addr: String },
    #[returns(QueryDelegatorUnbondingDelegationsResponse)]
    GetUnbondingDelegations { delegator_addr: String },
    #[returns(QueryShowCommitmentsResponse)]
    GetCommitments { delegator_addr: String },
    #[returns(GetUsdcPriceResp)]
    GetUsdcPrice {},
    #[returns(ContractVersion)]
    Version {},
}
