use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult, Decimal, Coin};

use super::{
    query::ElysQuery,
    query_resp::*,
};

use crate::msg::query_resp::earn::QueryEarnPoolResponse;
use crate::types::{BalanceBorrowed, QueryAprResponse, PageRequest};
use elys_bindings::{types::BalanceAvailable, query_resp::*};

#[allow(dead_code)]
pub struct ElysQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ElysQuery>,
}

#[allow(dead_code)]
impl<'a> ElysQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, ElysQuery>) -> Self {
        ElysQuerier { querier }
    }

    pub fn get_balance(&self, address: String, denom: String) -> StdResult<BalanceAvailable> {
        let balance_query = ElysQuery::AmmBalance {
            address: address.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(balance_query);
        let resp: BalanceAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_staked_balance(&self, address: String, denom: String)-> StdResult<StakedAvailable> {
        let staked_balance_query = ElysQuery::CommitmentStakedBalanceOfDenom{
            address: address.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(staked_balance_query);
        let resp: StakedAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_delegations(&self, delegator_addr: String) -> StdResult<QueryDelegatorDelegationsResponse> {
        let delegations_query = ElysQuery::CommitmentDelegations {
            delegator_address: delegator_addr.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(delegations_query);
        let resp: QueryDelegatorDelegationsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_unbonding_delegations(&self, delegator_addr: String) -> StdResult<QueryDelegatorUnbondingDelegationsResponse> {
        let unbonding_delegations_query = ElysQuery::CommitmentUnbondingDelegations {
            delegator_address: delegator_addr.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(unbonding_delegations_query);
        let resp: QueryDelegatorUnbondingDelegationsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_all_validators(&self, delegator: String) -> StdResult<QueryDelegatorValidatorsResponse> {
        let validators_query = ElysQuery::CommitmentAllValidators{ 
            delegator_address: delegator.to_owned()
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(validators_query);
        let resp: QueryDelegatorValidatorsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_delegator_validators(&self, delegator: String) -> StdResult<QueryDelegatorValidatorsResponse> {
        let validators_query = ElysQuery::CommitmentDelegatorValidators{
            delegator_address: delegator.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(validators_query);
        let resp: QueryDelegatorValidatorsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_commitments(&self, address: String) -> StdResult<QueryShowCommitmentsResponse> {
        let commitments_query = ElysQuery::CommitmentShowCommitments{
            creator: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(commitments_query);
        let resp: QueryShowCommitmentsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_rewards_balance(&self, address: String, denom: String) -> StdResult<BalanceAvailable> {
        let rewards_balance_query = ElysQuery::CommitmentRewardsBalanceOfDenom{
            address: address.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(rewards_balance_query);
        let resp: BalanceAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_borrowed_balance(&self, address: String) -> StdResult<BalanceBorrowed> {
        let borrowed_balance_query = ElysQuery::StableStakeBalanceOfBorrow{
            address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(borrowed_balance_query);
        let resp: BalanceBorrowed = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_staked_positions(&self, address: String) -> StdResult<QueryStakedPositionResponse> {
        let staked_position_query = ElysQuery::CommitmentStakedPositions{
            delegator_address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(staked_position_query);
        let resp: QueryStakedPositionResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_unstaked_positions(&self, address: String) -> StdResult<QueryUnstakedPositionResponse> {
        let unstaked_position_query = ElysQuery::CommitmentUnStakedPositions{
            delegator_address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(unstaked_position_query);
        let resp: QueryUnstakedPositionResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_vesting_info(&self, address: String) -> StdResult<QueryVestingInfoResponse> {
        let vesting_info_query = ElysQuery::CommitmentVestingInfo{
            address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(vesting_info_query);
        let resp: QueryVestingInfoResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_sub_bucket_rewards_balance(&self, address: String, denom: String, program: i32) -> StdResult<BalanceAvailable> {
        let sub_bucket_reward_query = ElysQuery::CommitmentRewardsSubBucketBalanceOfDenom{
            address: address.to_owned(),
            denom: denom.to_owned(),
            program: program.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(sub_bucket_reward_query);
        let resp: BalanceAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_incentive_apr(&self, program: i32, denom: String, ) -> StdResult<QueryAprResponse> {
        let incentive_apr_query = ElysQuery::IncentiveApr{
            withdraw_type: program.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(incentive_apr_query);
        let resp: QueryAprResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_amm_price_by_denom(&self, token_in: Coin, discount: Decimal ) -> StdResult<Decimal> {
        let amm_price_query = ElysQuery::AmmPriceByDenom{
            token_in: token_in.to_owned(),
            discount: discount.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(amm_price_query);
        let resp: Decimal = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_oracle_price(&self, asset: String, source: String, timestamp: u64 ) -> StdResult<QueryGetPriceResponse> {
        let oracle_price_query = ElysQuery::OraclePrice{
            asset: asset.to_owned(),
            source: source.to_owned(),
            timestamp: timestamp.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(oracle_price_query);
        let resp: QueryGetPriceResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_all_pools(&self, pool_ids: Option<Vec<u64>>, filter_type: i32, pagination: Option<PageRequest>) -> StdResult<QueryEarnPoolResponse> {
        let pools_query = ElysQuery::get_all_pools(pool_ids, filter_type, pagination);
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(pools_query);

        let resp: QueryEarnPoolResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_asset_profile(&self, base_denom: String ) -> StdResult<QueryGetEntryResponse> {
        let asset_profile = ElysQuery::get_asset_profile(base_denom.to_owned());
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(asset_profile);
        let QueryGetEntryResponseRaw { entry: raw_entry }: QueryGetEntryResponseRaw =
            self.querier.query(&request)?;

        let resp = QueryGetEntryResponse {
            entry: Entry {
                base_denom: raw_entry.base_denom,
                decimals: raw_entry.decimals,
                denom: raw_entry.denom,
                path: raw_entry.path.map_or("".to_string(), |path| path),
                ibc_channel_id: raw_entry
                    .ibc_channel_id
                    .map_or("".to_string(), |ibc_channel_id| ibc_channel_id),
                ibc_counterparty_channel_id: raw_entry
                    .ibc_counterparty_channel_id
                    .map_or("".to_string(), |ibc_counterparty_channel_id| {
                        ibc_counterparty_channel_id
                    }),
                display_name: raw_entry.display_name,
                display_symbol: raw_entry
                    .display_symbol
                    .map_or("".to_string(), |display_symbol| display_symbol),
                external_symbol: raw_entry
                    .external_symbol
                    .map_or("".to_string(), |external_symbol| external_symbol),
                unit_denom: raw_entry
                    .unit_denom
                    .map_or("".to_string(), |unit_denom| unit_denom),
                authority: raw_entry.authority,
                commit_enabled: raw_entry.commit_enabled,
                withdraw_enabled: raw_entry.withdraw_enabled,
                network: raw_entry.network.map_or("".to_string(), |network| network),
                address: raw_entry.address.map_or("".to_string(), |address| address),
                transfer_limit: raw_entry
                    .transfer_limit
                    .map_or("".to_string(), |transfer_limit| transfer_limit),
                ibc_counterparty_denom: raw_entry
                    .ibc_counterparty_denom
                    .map_or("".to_string(), |ibc_counterparty_denom| {
                        ibc_counterparty_denom
                    }),
                ibc_counterparty_chain_id: raw_entry
                    .ibc_counterparty_chain_id
                    .map_or("".to_string(), |ibc_counterparty_chain_id| {
                        ibc_counterparty_chain_id
                    }),
                permissions: raw_entry
                    .permissions
                    .map_or(vec![], |permissions| permissions),
            },
        };
        Ok(resp)
    }
}
