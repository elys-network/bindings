use std::{collections::HashMap, str::FromStr};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Coin, Decimal, Decimal256, Int128, SignedDecimal, SignedDecimal256, StdError, StdResult,
    Uint128,
};

use crate::{
    trade_shield::types::{
        AmmPool, AmmPoolRaw, CoinValue, PerpetualPosition, PoolExtraInfo, StakedPositionRaw,
    },
    types::{
        BalanceAvailable, Mtp, MtpAndPrice, OracleAssetInfo, PageResponse, PoolAsset, Price,
        StakedPosition, SwapAmountInRoute, SwapAmountOutRoute, UnstakedPosition, ValidatorDetail,
        VestingDetail,
    },
    ElysQuerier,
};

#[cw_serde]
pub struct OracleAllPriceResponse {
    pub price: Option<Vec<Price>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct AmmGetPoolResponseRaw {
    pub pool: AmmPoolRaw,
    pub extra_info: Option<PoolExtraInfo>,
}

#[cw_serde]
pub struct AmmGetPoolResponse {
    pub pool: AmmPool,
    pub extra_info: PoolExtraInfo,
}

#[cw_serde]
pub struct AmmGetPoolsResponseRaw {
    pub pool: Option<Vec<AmmPoolRaw>>,
    pub extra_infos: Option<Vec<PoolExtraInfo>>,
    pub pagination: Option<PageResponse>,
}

#[cw_serde]
pub struct AmmGetPoolsResponse {
    pub pool: Vec<AmmPool>,
    pub extra_infos: Vec<PoolExtraInfo>,
    pub pagination: Option<PageResponse>,
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
    pub mtps: Option<Vec<MtpAndPrice>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct PerpetualMtpResponse {
    pub mtp: Option<MtpAndPrice>,
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
pub struct CoinNeg {
    pub denom: String,
    pub amount: Int128,
}

impl Default for CoinNeg {
    fn default() -> Self {
        Self {
            denom: "".to_string(),
            amount: Int128::zero(),
        }
    }
}

#[cw_serde]
pub struct PerpetualOpenEstimationRawResponse {
    pub position: Option<i32>,
    pub leverage: Option<String>,
    pub trading_asset: Option<String>,
    pub collateral: Option<Coin>,
    pub interest_amount: Option<Int128>,
    pub position_size: Option<Coin>,
    pub swap_fee: Option<String>,
    pub discount: Option<String>,
    pub open_price: Option<String>,
    pub take_profit_price: Option<String>,
    pub liquidation_price: Option<String>,
    pub estimated_pnl: Option<Int128>,
    pub estimated_pnl_denom: Option<String>,
    pub available_liquidity: Option<Coin>,
    pub slippage: Option<String>,
    pub weight_balance_ratio: Option<String>,
    pub borrow_interest_rate: Option<String>,
    pub funding_rate: Option<String>,
    pub price_impact: Option<String>,
    pub borrow_fee: Option<Coin>,
    pub funding_fee: Option<CoinNeg>,
}

#[cw_serde]
pub struct PerpetualOpenEstimationResponse {
    pub position: PerpetualPosition,
    pub leverage: SignedDecimal,
    pub trading_asset: String,
    pub collateral: Coin,
    pub interest_amount: Int128,
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
    pub borrow_fee: Coin,
    pub funding_fee: CoinNeg,
}

impl Into<StdResult<PerpetualOpenEstimationResponse>> for PerpetualOpenEstimationRawResponse {
    fn into(self) -> StdResult<PerpetualOpenEstimationResponse> {
        Ok(PerpetualOpenEstimationResponse {
            position: PerpetualPosition::try_from_i32(self.position.unwrap_or_default())?,
            leverage: SignedDecimal::from_str(self.leverage.unwrap_or_default().as_str())
                .unwrap_or_default(),
            trading_asset: self.trading_asset.unwrap_or_default(),
            collateral: self.collateral.unwrap_or_default(),
            interest_amount: self.interest_amount.unwrap_or_default(),
            position_size: self.position_size.unwrap_or_default(),
            swap_fee: Decimal::from_str(self.swap_fee.unwrap_or_default().as_str())
                .unwrap_or_default(),
            discount: Decimal::from_str(self.discount.unwrap_or_default().as_str())
                .unwrap_or_default(),
            open_price: Decimal::from_str(self.open_price.unwrap_or_default().as_str())
                .unwrap_or_default(),
            take_profit_price: SignedDecimal256::from_str(
                self.take_profit_price.unwrap_or_default().as_str(),
            )
            .unwrap_or_default(),
            liquidation_price: SignedDecimal::from_str(
                self.liquidation_price.unwrap_or_default().as_str(),
            )
            .unwrap_or_default(),
            estimated_pnl: self.estimated_pnl.unwrap_or_default(),
            estimated_pnl_denom: self.estimated_pnl_denom.unwrap_or_default(),
            available_liquidity: self.available_liquidity.unwrap_or_default(),
            slippage: Decimal::from_str(self.slippage.unwrap_or_default().as_str())
                .unwrap_or_default(),
            weight_balance_ratio: SignedDecimal::from_str(
                self.weight_balance_ratio.unwrap_or_default().as_str(),
            )
            .unwrap_or_default(),
            borrow_interest_rate: SignedDecimal::from_str(
                self.borrow_interest_rate.unwrap_or_default().as_str(),
            )
            .unwrap_or_default(),
            funding_rate: SignedDecimal::from_str(self.funding_rate.unwrap_or_default().as_str())
                .unwrap_or_default(),
            price_impact: SignedDecimal::from_str(self.price_impact.unwrap_or_default().as_str())
                .unwrap_or_default(),
            borrow_fee: self.borrow_fee.unwrap_or_default(),
            funding_fee: self.funding_fee.unwrap_or(CoinNeg {
                denom: "".to_string(),
                amount: Int128::zero(),
            }),
        })
    }
}

#[cw_serde]
pub struct PerpetualGetPositionsForAddressResponseRaw {
    pub mtps: Option<Vec<MtpAndPrice>>,
    pub pagination: PageResponse,
}

#[cw_serde]
pub struct PerpetualGetPositionsForAddressResponse {
    pub mtps: Vec<MtpAndPrice>,
    pub pagination: PageResponse,
}

impl PerpetualGetPositionsForAddressResponse {
    pub fn get_mtp_vec(&self) -> Vec<Mtp> {
        self.mtps
            .iter()
            .map(|mtp_and_price| mtp_and_price.get_mtp())
            .collect()
    }
}

#[cw_serde]
pub struct TierCalculateDiscountResponse {
    pub discount: String,
    pub tier: String,
    pub portfolio: String,
}

#[cw_serde]
pub struct TierCalculateDiscountResponseRaw {
    pub discount: Option<String>,
    pub tier: Option<String>,
    pub portfolio: Option<String>,
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

#[derive(Default)]
#[cw_serde]
pub struct QueryAprsResponse {
    pub usdc_apr_usdc: Uint128,
    pub eden_apr_usdc: Uint128,
    pub usdc_apr_edenb: Uint128,
    pub eden_apr_edenb: Uint128,
    pub usdc_apr_eden: Uint128,
    pub eden_apr_eden: Uint128,
    pub edenb_apr_eden: Uint128,
    pub usdc_apr_elys: Uint128,
    pub eden_apr_elys: Uint128,
    pub edenb_apr_elys: Uint128,
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
pub struct BalanceBorrowedRaw {
    pub usd_amount: String,
    pub percentage: String,
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
pub struct VestingTokensRaw {
    pub denom: String,
    pub total_amount: Int128,
    pub claimed_amount: Int128,
    pub num_blocks: Option<i64>,
    pub start_block: Option<i64>,
    pub vest_started_timestamp: Option<i64>,
}

#[cw_serde]
pub struct VestingTokens {
    pub denom: String,
    pub total_amount: Int128,
    pub claimed_amount: Int128,
    pub num_blocks: i64,
    pub start_block: i64,
    pub vest_started_timestamp: i64,
}

#[cw_serde]
pub struct CommitmentsRaw {
    pub creator: String,
    pub committed_tokens: Option<Vec<CommittedTokens>>,
    pub rewards_unclaimed: Option<Vec<Coin>>,
    pub claimed: Option<Vec<Coin>>,
    pub vesting_tokens: Option<Vec<VestingTokensRaw>>,
    pub rewards_by_elys_unclaimed: Option<Vec<Coin>>,
    pub rewards_by_eden_unclaimed: Option<Vec<Coin>>,
    pub rewards_by_edenb_unclaimed: Option<Vec<Coin>>,
    pub rewards_by_usdc_unclaimed: Option<Vec<Coin>>,
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
pub struct QueryShowCommitmentsResponseRaw {
    pub commitments: CommitmentsRaw,
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
pub struct QueryJoinPoolEstimationResponse {
    amounts_in: Vec<Coin>,
    share_amount_out: Coin,
    slippage: Decimal,
    weight_balance_ratio: SignedDecimal,
}

#[cw_serde]
pub struct QueryPoolAssetEstimationResponse {
    pub amounts: HashMap<String, Decimal256>,
}

#[cw_serde]
pub struct QueryExitPoolEstimationResponse {
    pub amounts_out: Vec<Coin>,
}

#[cw_serde]
pub struct QueryUserPoolResponse {
    // Total Rewards in fiat
    pub total_rewards: Decimal,
    // Breakdown of the total rewards including the fiat amount keyed by denom
    pub total_rewards_breakdown: HashMap<String, CoinValue>,
    // Rewards keyed by pool ID
    pub rewards_per_pool: HashMap<u64, Vec<CoinValue>>,
    pub pools: Vec<UserPoolResp>,
}

#[cw_serde]
pub struct PoolResp {
    pub pool_id: i64,
    pub apr: Option<PoolApr>,
    pub assets: Vec<PoolAsset>, // eg : [{{"denom":"uatom", "amount":"1000"}, "weight":"10"}, {{"denom":"uusdc", "amount":"100"}, "weight":"1"}, ...]
    // Expected pool ratio
    pub pool_ratio: String,
    // Current pool ratio
    pub current_pool_ratio: Option<HashMap<String, Decimal>>,
    pub current_pool_ratio_string: Option<String>,
    pub rewards_apr: Decimal,
    pub rewards_usd: Decimal,
    pub reward_coins: Vec<Coin>,
    // Reward coins with USD value in it. Mapped from reward_coins chain response.
    pub fiat_rewards: Option<Vec<CoinValue>>,
    pub borrow_apr: Decimal,
    pub leverage_lp: Decimal,
    pub perpetual: Decimal,
    pub lp_token_price: Option<Decimal>,
    pub tvl: Decimal,
    pub total_shares: Coin,
    pub share_usd_price: Option<Decimal>,
    pub swap_fee: Decimal,
    pub fee_denom: String,
    pub use_oracle: Option<bool>,
    pub is_leveragelp: Option<bool>,
}

impl Default for PoolResp {
    fn default() -> Self {
        Self {
            pool_id: 0,
            apr: Some(PoolApr::default()),
            assets: vec![],
            pool_ratio: "".to_string(),
            current_pool_ratio: Some(HashMap::new()),
            current_pool_ratio_string: Some("".to_string()),
            rewards_apr: Decimal::zero(),
            borrow_apr: Decimal::zero(),
            leverage_lp: Decimal::zero(),
            perpetual: Decimal::zero(),
            tvl: Decimal::zero(),
            rewards_usd: Decimal::zero(),
            reward_coins: [Coin::new(0 as u128, "".to_string())].to_vec(),
            fiat_rewards: None,
            total_shares: Coin::new(0 as u128, "".to_string()),
            share_usd_price: Some(Decimal::zero()),
            fee_denom: "".to_string(),
            swap_fee: Decimal::zero(),
            use_oracle: Some(false),
            lp_token_price: None,
            is_leveragelp: None,
        }
    }
}

#[cw_serde]
pub struct UserPoolResp {
    pub pool: PoolResp,
    // User shares in pool
    pub balance: CommittedTokens,
    // User total balance in pool in terms of USD
    pub available: Decimal,
    // Balance based on current pool ratio
    pub balance_breakdown: Vec<Option<CoinValue>>,
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
#[derive(Default)]
pub struct PerpetualParamsRaw {
    pub leverage_max: Option<Decimal>,
    pub borrow_interest_rate_max: Option<Decimal>,
    pub borrow_interest_rate_min: Option<Decimal>,
    pub min_borrow_interest_amount: Option<Int128>,
    pub borrow_interest_rate_increase: Option<Decimal>,
    pub borrow_interest_rate_decrease: Option<Decimal>,
    pub health_gain_factor: Option<Decimal>,
    pub epoch_length: Option<i64>,
    pub max_open_positions: Option<i64>,
    pub pool_open_threshold: Option<Decimal>,
    pub force_close_fund_percentage: Option<Decimal>,
    pub force_close_fund_address: Option<String>,
    pub incremental_borrow_interest_payment_fund_percentage: Option<Decimal>,
    pub incremental_borrow_interest_payment_fund_address: Option<String>,
    pub safety_factor: Option<Decimal>,
    pub incremental_borrow_interest_payment_enabled: Option<bool>,
    pub whitelisting_enabled: Option<bool>,
    pub invariant_check_epoch: Option<String>,
    pub take_profit_borrow_interest_rate_min: Option<Decimal>,
    pub funding_fee_base_rate: Option<Decimal>,
    pub funding_fee_max_rate: Option<Decimal>,
    pub funding_fee_min_rate: Option<String>,
    pub funding_fee_collection_address: Option<String>,
    pub swap_fee: Option<Decimal>,
    pub max_limit_order: Option<i64>,
}

#[cw_serde]
pub struct PerpetualParams {
    pub leverage_max: Decimal,
    pub borrow_interest_rate_max: Decimal,
    pub borrow_interest_rate_min: Decimal,
    pub min_borrow_interest_amount: Int128,
    pub borrow_interest_rate_increase: Decimal,
    pub borrow_interest_rate_decrease: Decimal,
    pub health_gain_factor: Decimal,
    pub epoch_length: i64,
    pub max_open_positions: i64,
    pub pool_open_threshold: Decimal,
    pub force_close_fund_percentage: Decimal,
    pub force_close_fund_address: String,
    pub incremental_borrow_interest_payment_fund_percentage: Decimal,
    pub incremental_borrow_interest_payment_fund_address: String,
    pub safety_factor: Decimal,
    pub incremental_borrow_interest_payment_enabled: bool,
    pub whitelisting_enabled: bool,
    pub invariant_check_epoch: String,
    pub take_profit_borrow_interest_rate_min: Decimal,
    pub funding_fee_base_rate: Decimal,
    pub funding_fee_max_rate: Decimal,
    pub funding_fee_min_rate: String,
    pub funding_fee_collection_address: String,
    pub swap_fee: Decimal,
    pub max_limit_order: i64,
}

#[cw_serde]
pub struct LeveragelpParamsResponseRaw {
    pub params: Option<LeveragelpParamsRaw>,
}

#[cw_serde]
pub struct PerpetualParamsResponseRaw {
    pub params: Option<PerpetualParamsRaw>,
}

#[cw_serde]
pub struct LeveragelpParamsResponse {
    pub params: Option<LeveragelpParams>,
}

#[cw_serde]
pub struct PerpetualParamsResponse {
    pub params: Option<PerpetualParams>,
}

#[cw_serde]
pub struct Position {
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
pub struct LeveragelpPosition {
    pub position: Position,
    pub interest_rate_hour: Decimal,
    pub interest_rate_hour_usd: Decimal,
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

impl LeveragelpPositionsResponseRaw {
    pub fn get_positions(&self) -> Vec<u64> {
        self.positions
            .clone()
            .unwrap_or(vec![])
            .iter()
            .map(|x| x.position.id)
            .collect()
    }
}

#[cw_serde]
pub struct LeveragelpPositionsResponse {
    pub positions: Vec<LeveragelpPosition>,
    pub pagination: Option<PageResponse>,
}

#[cw_serde]
pub struct LeveragelpPositionsAndRewardsResponse {
    pub positions: LeveragelpPositionWithReward,
    pub pagination: Option<PageResponse>,
}

#[cw_serde]
#[derive(Default)]
pub struct LeveragelpPositionWithReward {
    pub positions: Vec<LeveragelpPosition>,
    pub rewards: LeveragelpFiatRewards,
}

#[cw_serde]
#[derive(Default)]
pub struct LeveragelpFiatRewards {
    pub rewards: Vec<RewardInfoMappedToCoinValue>,
    pub total_rewards: Vec<CoinValue>,
}

#[cw_serde]
#[derive(Default)]
pub struct RewardInfoMappedToCoinValue {
    pub position_id: u64,
    pub reward: Vec<CoinValue>,
}

#[cw_serde]
pub struct LeveragelpStatusResponse {
    pub open_position_count: u64,
    pub lifetime_position_count: u64,
}

#[cw_serde]
pub struct LeveragelpStatusResponseRaw {
    pub open_position_count: Option<u64>,
    pub lifetime_position_count: Option<u64>,
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

#[cw_serde]
pub struct LeveragelpOpenEstimationResponse {
    position_size: Int128,
    weight_balance_ratio: Decimal,
    borrow_fee: Decimal,
}

#[cw_serde]
pub struct LeveragelpCloseEstimationResponse {
    liability: Int128,
    weight_balance_ratio: Decimal,
    amount_returned: Int128,
}

#[cw_serde]
pub struct IncentiveInfoRaw {
    pub eden_amount_per_year: Option<Int128>,
    pub distribution_start_block: Option<Int128>,
    pub total_blocks_per_year: Option<Int128>,
    pub blocks_distributed: Option<Int128>,
}

#[cw_serde]
pub struct IncentiveInfo {
    pub eden_amount_per_year: Int128,
    pub distribution_start_block: Int128,
    pub total_blocks_per_year: Int128,
    pub blocks_distributed: Int128,
}

#[cw_serde]
pub struct SupportedRewardDenomRaw {
    pub denom: Option<String>,
    pub min_amount: Option<Int128>,
}

#[cw_serde]
pub struct SupportedRewardDenom {
    pub denom: String,
    pub min_amount: Int128,
}

#[cw_serde]
pub struct MasterchefParamsRaw {
    pub lp_incentives: Option<IncentiveInfoRaw>,
    pub reward_portion_for_lps: Option<SignedDecimal>,
    pub reward_portion_for_stakers: Option<SignedDecimal>,
    pub max_eden_reward_apr_lps: Option<SignedDecimal>,
    pub supported_reward_denoms: Option<Vec<SupportedRewardDenomRaw>>,
    pub protocol_revenue_address: Option<String>,
}

#[cw_serde]
pub struct MasterchefParams {
    pub lp_incentives: IncentiveInfo,
    pub reward_portion_for_lps: SignedDecimal,
    pub reward_portion_for_stakers: SignedDecimal,
    pub max_eden_reward_apr_lps: SignedDecimal,
    pub supported_reward_denoms: Vec<SupportedRewardDenom>,
    pub protocol_revenue_address: String,
}

#[cw_serde]
pub struct MasterchefParamsResponseRaw {
    pub params: MasterchefParamsRaw,
}

#[cw_serde]
pub struct MasterchefParamsResponse {
    pub params: MasterchefParams,
}

#[cw_serde]
pub struct MasterchefPoolInfoRaw {
    pub pool_id: Option<u64>,
    pub reward_wallet: Option<String>,
    pub multiplier: Option<SignedDecimal>,
    pub eden_apr: Option<SignedDecimal>,
    pub dex_apr: Option<SignedDecimal>,
    pub gas_apr: Option<SignedDecimal>,
    pub external_incentive_apr: Option<SignedDecimal>,
    pub external_reward_denoms: Option<Vec<String>>,
}

#[cw_serde]
pub struct MasterchefPoolInfo {
    pub pool_id: u64,
    pub reward_wallet: String,
    pub multiplier: SignedDecimal,
    pub eden_apr: SignedDecimal,
    pub dex_apr: SignedDecimal,
    pub gas_apr: SignedDecimal,
    pub external_incentive_apr: SignedDecimal,
    pub external_reward_denoms: Vec<String>,
}

#[cw_serde]
pub struct MasterchefPoolInfoResponseRaw {
    pub pool_info: MasterchefPoolInfoRaw,
}

#[cw_serde]
pub struct MasterchefPoolInfoResponse {
    pub pool_info: MasterchefPoolInfo,
}

#[cw_serde]
#[derive(Default)]
pub struct MasterchefUserPendingRewardResponse {
    pub rewards: Vec<MasterchefUserPendingRewardData>,
    pub total_rewards: Vec<Coin>,
}

#[cw_serde]
#[derive(Default)]
pub struct MasterchefUserPendingRewardData {
    pub pool_id: u64,
    pub reward: Vec<Coin>,
}
#[cw_serde]
#[derive(Default)]
pub struct EstakingRewardsResponse {
    pub rewards: Vec<DelegationDelegatorReward>,
    pub total: Vec<Coin>,
}

#[cw_serde]
#[derive(Default)]
pub struct DelegationDelegatorReward {
    pub validator_address: String,
    pub reward: Vec<Coin>,
}

pub enum Validator {
    EdenBoost,
    Eden,
}

impl Validator {
    pub fn to_string(&self) -> String {
        match self {
            Validator::EdenBoost => {
                "elysvaloper1wajd6ekh9u37hyghyw4mme59qmjllzuyaceanm".to_string()
            }
            Validator::Eden => "elysvaloper1gnmpr8vvslp3shcq6e922xr0uq4aa2w5gdzht0".to_string(),
        }
    }
}

impl EstakingRewardsResponse {
    pub fn get_elys_validators(&self) -> Self {
        let excluded_validator_addresses = vec![
            Validator::EdenBoost.to_string(),
            Validator::Eden.to_string(),
        ];

        let rewards = self
            .rewards
            .iter()
            .filter(|delegation_reward| {
                !excluded_validator_addresses.contains(&delegation_reward.validator_address)
            })
            .cloned()
            .collect::<Vec<_>>();

        let total = self.compute_total(&rewards);

        EstakingRewardsResponse { rewards, total }
    }

    pub fn get_validator_rewards(&self, validator: Validator) -> Self {
        let rewards = vec![self
            .rewards
            .iter()
            .find(|delegation_reward| delegation_reward.validator_address == validator.to_string())
            .cloned()
            .unwrap_or_default()];

        let total = self.compute_total(&rewards);

        EstakingRewardsResponse { rewards, total }
    }

    pub fn to_coin_values(
        &self,
        querier: &ElysQuerier<'_>,
    ) -> StdResult<HashMap<String, CoinValue>> {
        let mut coin_values: HashMap<String, CoinValue> = HashMap::new();

        for coin in &self.total {
            let coin_value = CoinValue::from_coin(&coin.clone(), querier).map_err(|e| {
                StdError::generic_err(format!("Failed to convert total to CoinValue: {}", e))
            })?;
            coin_values.insert(coin.denom.clone(), coin_value);
        }

        Ok(coin_values)
    }

    fn compute_total(&self, delegation_reward: &[DelegationDelegatorReward]) -> Vec<Coin> {
        let mut total_map: HashMap<String, u128> = HashMap::new();
        for delegation in delegation_reward {
            for coin in &delegation.reward {
                total_map
                    .entry(coin.denom.clone())
                    .and_modify(|existing_amount| {
                        *existing_amount = existing_amount
                            .checked_add(coin.amount.into())
                            .unwrap_or_default();
                    })
                    .or_insert(coin.amount.into());
            }
        }

        total_map
            .into_iter()
            .map(|(denom, amount)| Coin {
                denom,
                amount: amount.into(),
            })
            .collect()
    }
}

impl MasterchefUserPendingRewardResponse {
    pub fn to_coin_values(
        &self,
        querier: &ElysQuerier<'_>,
    ) -> StdResult<(HashMap<u64, Vec<CoinValue>>, Vec<CoinValue>)> {
        Ok((
            self.rewards_to_coin_values(querier)?,
            self.total_rewards_to_coin(querier)?,
        ))
    }

    pub fn rewards_to_coin_values(
        &self,
        querier: &ElysQuerier<'_>,
    ) -> StdResult<HashMap<u64, Vec<CoinValue>>> {
        let mut coin_values = HashMap::new();
        for MasterchefUserPendingRewardData { reward, pool_id } in &self.rewards {
            let coin = { coin_values.entry(*pool_id).or_insert_with(|| vec![]) };
            coin.extend(
                reward
                    .iter()
                    .map(|v| CoinValue::from_coin(v, querier).unwrap_or_default()),
            );
        }
        Ok(coin_values)
    }

    fn total_rewards_to_coin(&self, querier: &ElysQuerier<'_>) -> StdResult<Vec<CoinValue>> {
        let mut coin_values = Vec::new();
        for reward in &self.total_rewards {
            coin_values.push(CoinValue::from_coin(reward, querier)?);
        }
        Ok(coin_values)
    }
}

#[cw_serde]
#[derive(Default)]
pub struct QueryPoolAprsResponse {
    pub data: Vec<PoolApr>,
}

impl QueryPoolAprsResponse {
    pub fn to_decimal(&self) -> Vec<PoolApr> {
        let mut aprs: Vec<PoolApr> = Vec::new();
        for apr in self.data.clone().iter_mut() {
            aprs.push(PoolApr {
                pool_id: apr.pool_id,
                eden_apr: apr.eden_apr * Decimal::from_str("100").unwrap(),
                usdc_apr: apr.usdc_apr * Decimal::from_str("100").unwrap(),
                total_apr: apr.total_apr * Decimal::from_str("100").unwrap(),
            })
        }

        aprs
    }
}

#[derive(Default)]
#[cw_serde]
pub struct PoolApr {
    pub pool_id: u64,
    pub eden_apr: Decimal,
    pub usdc_apr: Decimal,
    pub total_apr: Decimal,
}

#[derive(Default)]
#[cw_serde]
pub struct QueryStableStakeAprResponse {
    pub apr: Int128,
}

#[cw_serde]
pub struct CommitmentNumberOfCommitmentsResponseRaw {
    pub number: Option<i64>,
}

#[cw_serde]
pub struct CommitmentNumberOfCommitmentsResponse {
    pub number: i64,
}

#[cw_serde]
pub struct GetLeverageLpRewardsResp {
    pub rewards: Vec<RewardInfo>,
    pub total_rewards: Vec<Coin>,
}

#[cw_serde]
pub struct RewardInfo {
    pub position_id: u64,
    pub reward: Vec<Coin>,
}

impl GetLeverageLpRewardsResp {
    pub fn total_rewards_to_coin_value(
        &self,
        querier: &ElysQuerier<'_>,
    ) -> StdResult<Vec<CoinValue>> {
        let mut coin_values = Vec::new();
        for reward in &self.total_rewards {
            coin_values.push(CoinValue::from_coin(reward, querier)?);
        }
        Ok(coin_values)
    }

    pub fn to_coin_value(
        &self,
        querier: &ElysQuerier<'_>,
    ) -> StdResult<Vec<RewardInfoMappedToCoinValue>> {
        let mut reward_info: Vec<RewardInfoMappedToCoinValue> = vec![];

        for reward in &self.rewards {
            let mut coin_values = vec![];
            for coin in reward.reward.clone() {
                coin_values.push(CoinValue::from_coin(&coin, querier)?);
            }
            reward_info.push(RewardInfoMappedToCoinValue {
                position_id: reward.position_id,
                reward: coin_values,
            })
        }
        Ok(reward_info)
    }
}

#[cw_serde]
pub struct ParameterParamsRaw {
    pub min_commission_rate: Option<Decimal>,
    pub max_voting_power: Option<Decimal>,
    pub min_self_delegation: Option<Int128>,
    pub broker_address: Option<String>,
    pub total_blocks_per_year: Option<u64>,
    pub rewards_data_lifetime: Option<u64>,
    pub wasm_max_label_size: Option<Int128>,
    pub wasm_max_size: Option<Int128>,
    pub wasm_max_proposal_wasm_size: Option<Int128>,
}

#[cw_serde]
pub struct ParameterParams {
    pub min_commission_rate: Decimal,
    pub max_voting_power: Decimal,
    pub min_self_delegation: Int128,
    pub broker_address: String,
    pub total_blocks_per_year: u64,
    pub rewards_data_lifetime: u64,
    pub wasm_max_label_size: Int128,
    pub wasm_max_size: Int128,
    pub wasm_max_proposal_wasm_size: Int128,
}
#[cw_serde]
pub struct ParameterParamsResponseRaw {
    pub params: ParameterParamsRaw,
}

#[cw_serde]
pub struct ParameterParamsResponse {
    pub params: ParameterParams,
}

impl Into<PerpetualParams> for PerpetualParamsRaw {
    fn into(self) -> PerpetualParams {
        PerpetualParams {
            min_borrow_interest_amount: self.min_borrow_interest_amount.unwrap_or_default(),
            leverage_max: self.leverage_max.unwrap_or_default(),
            borrow_interest_rate_max: self.borrow_interest_rate_max.unwrap_or_default(),
            borrow_interest_rate_min: self.borrow_interest_rate_min.unwrap_or_default(),
            borrow_interest_rate_increase: self.borrow_interest_rate_increase.unwrap_or_default(),
            borrow_interest_rate_decrease: self.borrow_interest_rate_decrease.unwrap_or_default(),
            health_gain_factor: self.health_gain_factor.unwrap_or_default(),
            epoch_length: self.epoch_length.unwrap_or_default(),
            max_open_positions: self.max_open_positions.unwrap_or_default(),
            pool_open_threshold: self.pool_open_threshold.unwrap_or_default(),
            force_close_fund_percentage: self.force_close_fund_percentage.unwrap_or_default(),
            force_close_fund_address: self.force_close_fund_address.unwrap_or_default(),
            incremental_borrow_interest_payment_fund_percentage: self
                .incremental_borrow_interest_payment_fund_percentage
                .unwrap_or_default(),
            incremental_borrow_interest_payment_fund_address: self
                .incremental_borrow_interest_payment_fund_address
                .unwrap_or_default(),
            safety_factor: self.safety_factor.unwrap_or_default(),
            incremental_borrow_interest_payment_enabled: self
                .incremental_borrow_interest_payment_enabled
                .unwrap_or_default(),
            whitelisting_enabled: self.whitelisting_enabled.unwrap_or_default(),
            invariant_check_epoch: self.invariant_check_epoch.unwrap_or_default(),
            take_profit_borrow_interest_rate_min: self
                .take_profit_borrow_interest_rate_min
                .unwrap_or_default(),
            funding_fee_base_rate: self.funding_fee_base_rate.unwrap_or_default(),
            funding_fee_max_rate: self.funding_fee_max_rate.unwrap_or_default(),
            funding_fee_min_rate: self.funding_fee_min_rate.unwrap_or_default(),
            funding_fee_collection_address: self.funding_fee_collection_address.unwrap_or_default(),
            swap_fee: self.swap_fee.unwrap_or_default(),
            max_limit_order: self.max_limit_order.unwrap_or_default(),
        }
    }
}
