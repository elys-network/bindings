#[allow(unused_imports)]
use super::query_resp::*;
#[allow(unused_imports)]
use crate::types::{BalanceBorrowed, QueryAprResponse, PageRequest};
#[allow(unused_imports)]
use elys_bindings::types::BalanceAvailable;

#[allow(unused_imports)]
use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused_imports)]
use cosmwasm_std::{CustomQuery, Decimal, Coin};

#[allow(unused_imports)]
use crate::msg::query_resp::earn::QueryEarnPoolResponse;

use elys_bindings::query_resp::QueryGetEntryResponse;


#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    #[returns(BalanceAvailable)]
    AmmBalance { address: String, denom: String },
    #[returns(QueryDelegatorDelegationsResponse)]
    CommitmentDelegations { delegator_address: String},
    #[returns(QueryDelegatorUnbondingDelegationsResponse)]
    CommitmentUnbondingDelegations { delegator_address: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    CommitmentAllValidators { delegator_address: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    CommitmentDelegatorValidators { delegator_address: String },
    #[returns(BalanceAvailable)]
    CommitmentStakedBalanceOfDenom { address: String, denom: String },
    #[returns(BalanceAvailable)]
    CommitmentRewardsBalanceOfDenom { address: String, denom: String },
    #[returns(QueryShowCommitmentsResponse)]
    CommitmentShowCommitments { creator: String },
    #[returns(BalanceBorrowed)]
    StableStakeBalanceOfBorrow { address: String },
    #[returns(QueryStakedPositionResponse)]
    CommitmentStakedPositions { delegator_address: String },
    #[returns(QueryUnstakedPositionResponse)]
    CommitmentUnStakedPositions { delegator_address: String },
    #[returns(QueryVestingInfoResponse)]
    CommitmentVestingInfo{ address: String },
    #[returns(BalanceAvailable)]
    CommitmentRewardsSubBucketBalanceOfDenom{ address: String, denom: String, program: i32},
    #[returns(QueryAprResponse)]
    IncentiveApr{withdraw_type: i32, denom: String},
    #[returns(Decimal)]
    AmmPriceByDenom{token_in: Coin, discount: Decimal},
    #[returns(QueryGetPriceResponse)]
    OraclePrice{asset: String, source: String, timestamp: u64},
    #[returns(QueryEarnPoolResponse)]
    AmmEarnMiningPoolAll{pool_ids: Option<Vec<u64>>, filter_type: i32, pagination: Option<PageRequest>},
    #[returns(QueryGetEntryResponse)]
    AssetProfileEntry{base_denom: String}
}

impl CustomQuery for ElysQuery {}
impl ElysQuery {
    pub fn get_balance(address: String, denom: String) -> Self {
        ElysQuery::AmmBalance{ address, denom }
    }
    pub fn get_delegations(delegator_addr: String) -> Self {
        ElysQuery::CommitmentDelegations{ delegator_address: delegator_addr }
    }
    pub fn get_unbonding_delegations(delegator_addr: String) -> Self {
        ElysQuery::CommitmentUnbondingDelegations{ delegator_address: delegator_addr }
    }
    pub fn get_all_validators() -> Self {
        ElysQuery::CommitmentAllValidators{ delegator_address: "".to_string() }
    }
    pub fn get_delegator_validators(delegator_addr: String) -> Self {
        ElysQuery::CommitmentDelegatorValidators{ delegator_address: delegator_addr }
    }
    pub fn get_commitments(address: String) -> Self {
        ElysQuery::CommitmentShowCommitments{ creator: address }
    }
    pub fn get_staked_balance(address: String, denom: String) -> Self {
        ElysQuery::CommitmentStakedBalanceOfDenom{ address, denom }
    }
    pub fn get_rewards_balance(address: String, denom: String) -> Self {
        ElysQuery::CommitmentRewardsBalanceOfDenom{ address, denom }
    }
    pub fn get_borrowed_balance(address: String) -> Self {
        ElysQuery::StableStakeBalanceOfBorrow{ address }
    }
    pub fn get_staked_positions(delegator_addr: String) -> Self {
        ElysQuery::CommitmentStakedPositions{ delegator_address: delegator_addr }
    }
    pub fn get_unstaked_positions(delegator_addr: String) -> Self {
        ElysQuery::CommitmentUnStakedPositions{ delegator_address: delegator_addr }
    }
    pub fn get_vesting_info(address: String) -> Self {
        ElysQuery::CommitmentVestingInfo{ address }
    }
    pub fn get_sub_bucket_rewards_balance(address: String, denom: String, program: i32) -> Self {
        ElysQuery::CommitmentRewardsSubBucketBalanceOfDenom{ address, denom, program }
    }
    pub fn get_incentive_apr(program: i32, denom: String) -> Self {
        ElysQuery::IncentiveApr{ withdraw_type: program, denom }
    }
    pub fn get_amm_price_by_denom(token_in: Coin, discount: Decimal) -> Self {
        ElysQuery::AmmPriceByDenom{ token_in, discount }
    }
    pub fn get_oracle_price(asset: String, source: String, timestamp: u64) -> Self {
        ElysQuery::OraclePrice{ asset, source, timestamp }
    }
    pub fn get_all_pools(pool_ids: Option<Vec<u64>>, filter_type: i32, pagination: Option<PageRequest>) -> Self {
        ElysQuery::AmmEarnMiningPoolAll{ pool_ids, filter_type, pagination }
    }
    pub fn get_asset_profile(base_denom: String) -> Self {
        ElysQuery::AssetProfileEntry{ base_denom }
    }
}