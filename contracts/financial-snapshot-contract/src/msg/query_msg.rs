#[allow(unused_imports)]
use super::query_resp::earn::*;
#[allow(unused_imports)]
use super::query_resp::pod::*;
#[allow(unused_imports)]
use crate::bindings::query_resp::*;
#[allow(unused_imports)]
use crate::types::PageRequest;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Pod dashboard
    #[returns(GetRewardsResp)]
    GetPodRewards { user_address: String },
    #[returns(GetLiquidityPositionsResp)]
    GetPodLiquidityPositions {},
    #[returns(GetLiquidityPositionResp)]
    GetPodLiquidityPosition { pool_id: u64 },

    // Earn dashboard
    #[returns(GetEdenEarnProgramResp)]
    GetEdenEarnProgramDetails { user_address: Option<String> },
    #[returns(GetEdenBoostEarnProgramResp)]
    GetEdenBoostEarnProgramDetails { user_address: Option<String> },
    #[returns(GetElysEarnProgramResp)]
    GetElysEarnProgramDetails { user_address: Option<String> },
    #[returns(GetUsdcEarnProgramResp)]
    GetUsdcEarnProgramDetails { user_address: Option<String> },
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
    #[returns(QueryEarnPoolResponse)]
    GetLiquidityPools {
        pool_ids: Option<Vec<u64>>,
        filter_type: FilterType,
        pagination: Option<PageRequest>,
    },
    #[returns(GetUsdcPriceResp)]
    GetUsdcPrice {},
}
