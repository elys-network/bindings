use std::collections::HashMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Decimal256, Int128, SignedDecimal, SignedDecimal256, Uint128};

use crate::{
    trade_shield::types::{PerpetualPosition, StakedPositionRaw},
    types::{
        BalanceAvailable, Mtp, OracleAssetInfo, PageResponse, PoolAsset, Price, StakedPosition,
        SwapAmountInRoute, SwapAmountOutRoute, UnstakedPosition, ValidatorDetail, VestingDetail,
    },
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
    pub swap_fee: SignedDecimal,
    pub discount: Decimal,
    pub available_liquidity: Coin,
    pub slippage: Decimal,
    pub weight_balance_ratio: Decimal,
}

#[cw_serde]
pub struct OracleAssetInfoResponse {
    pub asset_info: OracleAssetInfo,
}

#[cw_serde]
pub struct PerpetualQueryPositionsResponse {
    pub mtps: Option<Vec<Mtp>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct PerpetualMtpResponse {
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
    pub slippage: Decimal,
}

#[cw_serde]
pub struct PerpetualOpenEstimationRawResponse {
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
    pub estimated_pnl_denom: String,
    pub available_liquidity: Coin,
    pub slippage: String,
    pub weight_balance_ratio: String,
    pub borrow_interest_rate: String,
    pub funding_rate: String,
    pub price_impact: String,
}

#[cw_serde]
pub struct PerpetualOpenEstimationResponse {
    pub position: PerpetualPosition,
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
    pub liquidation_price: SignedDecimal,
    pub estimated_pnl: Int128,
    pub estimated_pnl_denom: String,
    pub available_liquidity: Coin,
    pub slippage: Decimal,
    pub weight_balance_ratio: SignedDecimal,
    pub borrow_interest_rate: SignedDecimal,
    pub funding_rate: SignedDecimal,
    pub price_impact: SignedDecimal,
}

#[cw_serde]
pub struct PerpetualGetPositionsForAddressResponseRaw {
    pub mtps: Option<Vec<Mtp>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct PerpetualGetPositionsForAddressResponse {
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
pub struct QueryGetEntryAllResponse {
    pub pagination: PageResponse,
    pub entry: Option<Vec<Entry>>,
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

// implement default
impl Default for StakedAvailable {
    fn default() -> Self {
        Self {
            usd_amount: Decimal::zero(),
            amount: Uint128::zero(),
            lockups: None,
        }
    }
}

#[cw_serde]
pub struct QueryAprResponse {
    pub apr: Uint128,
}

impl Default for QueryAprResponse {
    fn default() -> Self {
        Self {
            apr: Uint128::zero(),
        }
    }
}

#[cw_serde]
pub struct QueryGetPriceResponse {
    pub price: Price,
}

#[cw_serde]
pub struct QueryStakedPositionResponseRaw {
    pub staked_position: Option<Vec<StakedPositionRaw>>,
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
pub struct StableStakeParamsResp {
    pub params: StableStakeParamsData,
}

#[cw_serde]
pub struct StableStakeParamsData {
    pub deposit_denom: String,
    pub redemption_rate: Decimal,
    pub epoch_length: i64,
    pub interest_rate: Decimal,
    pub interest_rate_max: Decimal,
    pub interest_rate_min: Decimal,
    pub interest_rate_increase: Decimal,
    pub interest_rate_decrease: Decimal,
    pub health_gain_factor: Decimal,
    pub total_value: Uint128,
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
    vest_started_timestamp: i64,
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
pub struct QueryIncentivePoolAprsResponse {
    pub data: Option<Vec<IncentivePoolApr>>,
}

#[cw_serde]
pub struct QueryJoinPoolEstimationResponse {
    amounts_in: Vec<Coin>,
    share_amount_out: Coin,
    slippage: Decimal,
    weight_balance_ratio: SignedDecimal,
}

#[cw_serde]
pub struct QueryPoolAssetEstimationResponse {
    pub amounts: HashMap<String, Decimal256>
}

#[cw_serde]
pub struct QueryUserPoolResponse {
    pub pools: Vec<UserPoolResp>,
}

#[cw_serde]
pub struct PoolResp {
    pub pool_id: i64,
    pub apr: Option<Decimal>,
    pub assets: Vec<PoolAsset>, // eg : [{{"denom":"uatom", "amount":"1000"}, "weight":"10"}, {{"denom":"uusdc", "amount":"100"}, "weight":"1"}, ...]
    // Expected pool ratio
    pub pool_ratio: String,
    // Current pool ratio
    pub current_pool_ratio: Option<HashMap<String, Decimal>>,
    pub current_pool_ratio_string: Option<String>,
    pub rewards_apr: Decimal,
    pub rewards_usd: Option<Decimal>,
    pub reward_coins: Option<Vec<Coin>>,
    pub borrow_apr: Decimal,
    pub leverage_lp: Decimal,
    pub perpetual: Decimal,
    pub tvl: Decimal,
    pub total_shares: Coin,
    pub share_usd_price: Option<Decimal>,
    pub swap_fee: Option<Decimal>,
    pub fee_denom: Option<String>,
    pub use_oracle: Option<bool>
}

#[cw_serde]
pub struct IncentivePoolApr {
    pub apr: Decimal,
    pub pool_id: i64,
}

#[cw_serde]
pub struct UserPoolResp {
    pub pool: PoolResp,
    // User shares in pool
    pub balance: CommittedTokens,
    // User balance in pool in terms of USD
    pub available: Decimal
}

#[cw_serde]
pub enum PoolFilterType {
    FilterAll = 0,
    FilterPerpetual = 1,
    FilterFixedWeight = 2,
    FilterDynamicWeight = 3,
    FilterLeverage = 4,
}

#[cw_serde]
pub struct LeveragelpParamsRaw {
    pub leverage_max: Option<Decimal>,
    pub max_open_positions: Option<i64>,
    pub pool_open_threshold: Option<Decimal>,
    pub safety_factor: Option<Decimal>,
    pub whitelisting_enabled: Option<bool>,
    pub epoch_length: Option<i64>,
}

#[cw_serde]
pub struct LeveragelpParams {
    pub leverage_max: Decimal,
    pub max_open_positions: i64,
    pub pool_open_threshold: Decimal,
    pub safety_factor: Decimal,
    pub whitelisting_enabled: bool,
    pub epoch_length: i64,
}

#[cw_serde]
pub struct LeveragelpParamsResponseRaw {
    pub params: Option<LeveragelpParamsRaw>,
}

#[cw_serde]
pub struct LeveragelpParamsResponse {
    pub params: Option<LeveragelpParams>,
}

#[cw_serde]
pub struct LeveragelpPosition {
    pub address: String,
    pub collateral: Coin,
    pub liabilities: Int128,
    pub interest_paid: Int128,
    pub leverage: Decimal,
    pub leveraged_lp_amount: Int128,
    pub position_health: Decimal,
    pub id: u64,
    pub amm_pool_id: u64,
    pub stop_loss_price: Decimal,
}

#[cw_serde]
pub struct LeveragelpPositionResponse {
    pub position: Option<LeveragelpPosition>,
}

#[cw_serde]
pub struct LeveragelpPositionsResponseRaw {
    pub positions: Option<Vec<LeveragelpPosition>>,
    pub pagination: Option<PageResponse>,
}

#[cw_serde]
pub struct LeveragelpPositionsResponse {
    pub positions: Vec<LeveragelpPosition>,
    pub pagination: Option<PageResponse>,
}

#[cw_serde]
pub struct LeveragelpStatusReponse {
    pub open_position_count: u64,
    pub lifetime_position_count: u64,
}

#[cw_serde]
pub struct LeveragelpWhitelistResponseRaw {
    pub whitelist: Option<Vec<String>>,
    pub pagination: Option<PageResponse>,
}


#[cw_serde]
pub struct LeveragelpWhitelistResponse {
    pub whitelist: Vec<String>,
    pub pagination: Option<PageResponse>,
}

#[cw_serde]
pub struct LeveragelpIsWhitelistedResponseRaw {
    pub address: String,
    pub is_whitelisted: Option<bool>,
}

#[cw_serde]
pub struct LeveragelpIsWhitelistedResponse {
    pub address: String,
    pub is_whitelisted: bool,
}

#[cw_serde]
pub struct LeveragelpPoolRaw {
    pub amm_pool_id: u64,
    pub health: Decimal,
    pub enabled: Option<bool>,
    pub closed: Option<bool>,
    pub leveraged_lp_amount: Int128,
    pub leverage_max: Decimal,
}

#[cw_serde]
pub struct LeveragelpPool {
    pub amm_pool_id: u64,
    pub health: Decimal,
    pub enabled: bool,
    pub closed: bool,
    pub leveraged_lp_amount: Int128,
    pub leverage_max: Decimal,
}

#[cw_serde]
pub struct LeveragelpPoolResponseRaw {
    pub pool: LeveragelpPoolRaw,
}

#[cw_serde]
pub struct LeveragelpPoolResponse {
    pub pool: LeveragelpPool,
}

#[cw_serde]
pub struct LeveragelpPoolsResponseRaw {
    pub pool: Vec<LeveragelpPoolRaw>,
    pub pagination: Option<PageResponse>,
}

#[cw_serde]
pub struct LeveragelpPoolsResponse {
    pub pool: Vec<LeveragelpPool>,
    pub pagination: Option<PageResponse>,
}
