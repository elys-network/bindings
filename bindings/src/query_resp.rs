use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Int128, SignedDecimal, SignedDecimal256, Uint128};

use crate::types::{
    BalanceAvailable, Mtp, OracleAssetInfo, PageResponse, PoolAsset, Price, StakedPosition,
    SwapAmountInRoute, SwapAmountOutRoute, UnstakedPosition, ValidatorDetail, VestingDetail,
};

#[cw_serde]
pub struct OracleAllPriceResponse {
    pub price: Option<Vec<Price>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct AmmSwapEstimationResponse {
    pub spot_price: Decimal,
    pub token_out: Coin,
    pub discount: Decimal,
    pub swap_fee: SignedDecimal,
    pub available_liquidity: Coin,
}

#[cw_serde]
pub struct OracleAssetInfoResponse {
    pub asset_info: OracleAssetInfo,
}

#[cw_serde]
pub struct MarginQueryPositionsResponse {
    pub mtps: Option<Vec<Mtp>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct MarginMtpResponse {
    pub mtp: Option<Mtp>,
}

#[cw_serde]
pub struct AuthAddressesResponse {
    pub addresses: Vec<String>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct InRouteByDenomResponse {
    pub in_routes: Vec<SwapAmountInRoute>,
}

#[cw_serde]
pub struct AmmSwapEstimationByDenomResponse {
    pub in_route: Option<Vec<SwapAmountInRoute>>,
    pub out_route: Option<Vec<SwapAmountOutRoute>>,
    pub spot_price: Decimal,
    pub amount: Coin,
    pub swap_fee: SignedDecimal,
    pub discount: SignedDecimal,
    pub available_liquidity: Coin,
    pub weight_balance_ratio: SignedDecimal,
    pub price_impact: SignedDecimal,
}

#[cw_serde]
pub struct MarginOpenEstimationRawResponse {
    pub position: i32,
    pub leverage: String,
    pub trading_asset: String,
    pub collateral: Coin,
    pub min_collateral: Coin,
    pub valid_collateral: Option<bool>,
    pub position_size: Coin,
    pub swap_fee: String,
    pub discount: String,
    pub open_price: String,
    pub take_profit_price: String,
    pub liquidation_price: String,
    pub estimated_pnl: Int128,
    pub available_liquidity: Coin,
    pub weight_balance_ratio: String,
    pub borrow_interest_rate: String,
    pub funding_rate: String,
    pub price_impact: String,
}

#[cw_serde]
pub struct MarginOpenEstimationResponse {
    pub position: i32,
    pub leverage: SignedDecimal,
    pub trading_asset: String,
    pub collateral: Coin,
    pub min_collateral: Coin,
    pub valid_collateral: bool,
    pub position_size: Coin,
    pub swap_fee: Decimal,
    pub discount: Decimal,
    pub open_price: Decimal,
    pub take_profit_price: SignedDecimal256,
    pub liquidation_price: Decimal,
    pub estimated_pnl: Int128,
    pub available_liquidity: Coin,
    pub weight_balance_ratio: Decimal,
    pub borrow_interest_rate: Decimal,
    pub funding_rate: Decimal,
    pub price_impact: Decimal,
}

#[cw_serde]
pub struct MarginGetPositionsForAddressResponseRaw {
    pub mtps: Option<Vec<Mtp>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct MarginGetPositionsForAddressResponse {
    pub mtps: Vec<Mtp>,
    pub pagination: PageResponse,
}

// TODO:
// Some of the items are commented because of the omitempty flag in protobuf.
// Until we fix this problem, we keep the items commented to be same result from the live elys testnet asset entry.
#[cw_serde]
pub struct Entry {
    pub base_denom: String,
    pub decimals: u64,
    pub denom: String,
    pub path: String,
    pub ibc_channel_id: String,
    pub ibc_counterparty_channel_id: String,
    pub display_name: String,
    pub display_symbol: String,
    pub network: String,
    pub address: String,
    pub external_symbol: String,
    pub transfer_limit: String,
    pub permissions: Vec<String>,
    pub unit_denom: String,
    pub ibc_counterparty_denom: String,
    pub ibc_counterparty_chain_id: String,
    pub authority: String,
    pub commit_enabled: bool,
    pub withdraw_enabled: bool,
}

#[cw_serde]
pub struct RawEntry {
    pub base_denom: Option<String>,
    pub decimals: Option<u64>,
    pub denom: Option<String>,
    pub path: Option<String>,
    pub ibc_channel_id: Option<String>,
    pub ibc_counterparty_channel_id: Option<String>,
    pub display_name: Option<String>,
    pub display_symbol: Option<String>,
    pub network: Option<String>,
    pub address: Option<String>,
    pub external_symbol: Option<String>,
    pub transfer_limit: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub unit_denom: Option<String>,
    pub ibc_counterparty_denom: Option<String>,
    pub ibc_counterparty_chain_id: Option<String>,
    pub authority: Option<String>,
    pub commit_enabled: Option<bool>,
    pub withdraw_enabled: Option<bool>,
}

#[cw_serde]
pub struct QueryGetEntryResponseRaw {
    pub entry: RawEntry,
}

#[cw_serde]
pub struct QueryGetEntryResponse {
    pub entry: Entry,
}

#[cw_serde]
pub struct Lockup {
    pub amount: Int128,
    pub unlock_timestamp: u64,
}

#[cw_serde]
pub struct StakedAvailable {
    pub usd_amount: Decimal,
    pub amount: Uint128,
    pub lockups: Option<Vec<Lockup>>,
}

#[cw_serde]
pub struct QueryAprResponse {
    pub apr: Uint128,
}

#[cw_serde]
pub struct QueryGetPriceResponse {
    pub price: Price,
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
pub struct BalanceBorrowed {
    pub usd_amount: Decimal,
    pub percentage: Decimal,
}

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
pub struct QueryVestingInfoResponse {
    pub vesting: BalanceAvailable,
    pub vesting_details: Option<Vec<VestingDetail>>,
}

#[cw_serde]
pub struct QueryEarnPoolResponse {
    pub pools: Option<Vec<PoolResp>>,
}

#[cw_serde]
pub struct PoolResp {
    pub assets: Vec<PoolAsset>, // eg : [{{"denom":"uatom", "amount":"1000"}, "weight":"10"}, {{"denom":"uusdc", "amount":"100"}, "weight":"1"}, ...]
    pub pool_ratio: String,
    pub rewards_apr: Decimal,
    pub borrow_apr: Decimal,
    pub leverage_lp: Decimal,
    pub perpetual: Decimal,
    pub tvl: Decimal,
    pub rewards: Decimal,
}
