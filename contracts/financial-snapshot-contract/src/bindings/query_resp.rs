use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Int128, Uint128};
use crate::types::{ValidatorDetail, StakedPosition, UnstakedPosition, VestingDetail};
use elys_bindings::types::BalanceAvailable;

#[cw_serde]
pub struct QueryBalanceResponse {
    pub balance: Coin,
}

#[cw_serde]
pub struct Delegation {
	pub delegator_address: String,
    pub validator_address: String,
    pub shares: Decimal,
}

#[cw_serde]
pub struct DelegationResponse {
	pub delegation: Delegation,
    pub balance: Coin,
}

#[cw_serde]
pub struct QueryDelegatorDelegationsResponse {
    pub delegation_responses: Vec<DelegationResponse>,
}

#[cw_serde]
pub struct UnbondingDelegationEntry {
    pub balance: Int128,
    pub completion_time: i64,
    pub creation_height: i64,
    pub initial_balance: Int128,
    pub unbonding_id: u64,
}

#[cw_serde]
pub struct UnbondingDelegation {
	pub delegator_address: String,
    pub validator_address: String,
    pub entries: Option<Vec<UnbondingDelegationEntry>>,
}

#[cw_serde]
pub struct QueryDelegatorUnbondingDelegationsResponse {
    pub unbonding_responses: Option<Vec<UnbondingDelegation>>,
}

#[cw_serde]
pub struct QueryDelegatorValidatorsResponse {
    pub validators: Option<Vec<ValidatorDetail>>,
}

#[cw_serde]
pub struct Lockup {
	pub amount: Int128,
	pub unlock_timestamp: u64,
}

#[cw_serde]
pub struct CommittedTokens {
	pub denom: String,
	pub amount: Int128,
	pub lockups: Option<Vec<Lockup>>,
}

#[cw_serde]
pub struct RewardsUnclaimed {
	pub denom: String,
	pub amount: Int128,
}

#[cw_serde]
pub struct VestingTokens {
	denom: String,
	total_amount: Int128,
	unvested_amount: Int128,
	epoch_identifier: String,
	num_epochs: i64,
	current_epoch: i64,
}

#[cw_serde]
pub struct Commitments {
	pub creator: String,
	pub committed_tokens: Option<Vec<CommittedTokens>>,
	pub rewards_unclaimed: Option<Vec<Coin>>,
	pub claimed: Option<Vec<Coin>>,
	pub vesting_tokens: Option<Vec<VestingTokens>>,
	pub rewards_by_elys_unclaimed: Option<Vec<Coin>>,
	pub rewards_by_eden_unclaimed: Option<Vec<Coin>>,
	pub rewards_by_edenb_unclaimed: Option<Vec<Coin>>,
	pub rewards_by_usdc_unclaimed: Option<Vec<Coin>>,
}

#[cw_serde]
pub struct QueryShowCommitmentsResponse {
	pub commitments: Commitments,
}

#[cw_serde]
pub struct QueryStakedPositionResponse {
	pub staked_position: Option<Vec<StakedPosition>>,
}

#[cw_serde]
pub struct QueryUnstakedPositionResponse {
	pub unstaked_position: Option<Vec<UnstakedPosition>>,
}

#[cw_serde]
pub struct QueryVestingInfoResponse {
	pub vesting: BalanceAvailable,
	pub vesting_details: Option<Vec<VestingDetail>>,
}

#[cw_serde]
pub struct StakedAvailable {
	pub usd_amount: Decimal,
	pub amount: Uint128,
	pub lockups: Option<Vec<Lockup>>,
}

#[cw_serde]
pub struct Price {
	pub asset: String,
	pub price: Decimal,
	pub source: String,
	pub provider: String,
	pub timestamp: u64,
}

#[cw_serde]
pub struct QueryGetPriceResponse {
	pub price: Price,
}
