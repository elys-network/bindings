use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::from_json;
use cosmwasm_std::to_json_binary;
use cosmwasm_std::Binary;
use cosmwasm_std::Coin;
use cosmwasm_std::Decimal;
use cosmwasm_std::Int128;
use cosmwasm_std::Int256;
use cosmwasm_std::SignedDecimal;
use cosmwasm_std::SignedDecimal256;
use cosmwasm_std::StdError;
use cosmwasm_std::StdResult;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct OracleAssetInfo {
    pub denom: String,
    pub display: String,
    pub band_ticker: String,
    pub elys_ticker: String,
    pub decimal: u64,
}

#[cfg(feature = "testing")]
impl OracleAssetInfo {
    pub fn new(
        denom: String,
        display: String,
        band_ticker: String,
        elys_ticker: String,
        decimal: u64,
    ) -> Self {
        Self {
            denom,
            display,
            band_ticker,
            elys_ticker,
            decimal,
        }
    }

    pub fn default(denom: String, decimal: u64) -> Self {
        Self {
            denom,
            display: "".to_string(),
            band_ticker: "".to_string(),
            elys_ticker: "".to_string(),
            decimal,
        }
    }
}

#[cw_serde]
pub struct PageResponse {
    pub next_key: Option<Binary>,
    pub total: Option<u64>,
}

impl PageResponse {
    pub fn new(next_key: Option<Binary>, total: Option<u64>) -> Self {
        Self { next_key, total }
    }
    pub fn empty(count_total: bool) -> Self {
        Self {
            next_key: None,
            total: if count_total { Some(0) } else { None },
        }
    }
}

#[cw_serde]
pub struct PageRequest {
    pub key: Option<Binary>,
    pub offset: Option<u64>,
    pub limit: u64,
    pub count_total: bool,
    pub reverse: bool,
}

impl PageRequest {
    pub fn filter<T>(&self, static_vec: Vec<T>) -> StdResult<(Vec<T>, PageResponse)>
    where
        T: PartialEq,
        T: Clone,
    {
        let mut filter_vec = static_vec.clone();

        if self.reverse {
            filter_vec.reverse();
        };

        let key = if let Some(key) = &self.key {
            let key: u64 = from_json(key)?;
            if key >= filter_vec.len() as u64 {
                return Ok((vec![], PageResponse::empty(self.count_total)));
            } else {
                filter_vec = filter_vec.split_off(key as usize);
                key
            }
        } else {
            0
        };

        let offset = if let Some(offset) = self.offset {
            if offset >= filter_vec.len() as u64 {
                return Ok((vec![], PageResponse::empty(self.count_total)));
            } else {
                filter_vec = filter_vec.split_off(offset as usize);
                offset
            }
        } else {
            0
        };

        if filter_vec.is_empty() {
            return Ok((vec![], PageResponse::empty(self.count_total)));
        };

        if self.limit < filter_vec.len() as u64 {
            let _ = filter_vec.split_off(self.limit as usize);
        };

        if filter_vec.is_empty() {
            return Ok((vec![], PageResponse::empty(self.count_total)));
        };

        let next_key = if static_vec.last() == filter_vec.last() {
            None
        } else {
            Some(to_json_binary(&(key + self.limit + offset))?)
        };

        let total = if self.count_total {
            Some(static_vec.len() as u64)
        } else {
            None
        };

        let page_response = PageResponse::new(next_key, total);

        Ok((filter_vec, page_response))
    }
}

impl PageRequest {
    pub fn new(limit: u64) -> Self {
        Self {
            key: None,
            limit,
            offset: None,
            count_total: false,
            reverse: false,
        }
    }

    pub fn update(&mut self, key: Option<Binary>) -> () {
        self.key = key;
    }
}

#[cw_serde]
pub struct Price {
    pub asset: String,
    pub price: Decimal,
    pub source: String,
    pub provider: String,
    pub timestamp: u64,
    pub block_height: u64,
}

#[cfg(feature = "testing")]
impl Price {
    pub fn new(asset: impl Into<String>, price: Decimal) -> Price {
        Price {
            asset: asset.into(),
            price,
            source: "".to_string(),
            provider: "".to_string(),
            timestamp: 0,
            block_height: 0,
        }
    }
}

#[cw_serde]
pub struct SwapAmountInRoute {
    pub pool_id: u64,
    pub token_out_denom: String,
}

#[cfg(feature = "testing")]
impl SwapAmountInRoute {
    pub fn new(pool_id: u64, token_out_denom: impl Into<String>) -> Self {
        Self {
            pool_id,
            token_out_denom: token_out_denom.into(),
        }
    }
}

#[cw_serde]
pub struct SwapAmountOutRoute {
    pub pool_id: u64,
    pub token_in_denom: String,
}

#[cw_serde]
pub enum PerpetualPosition {
    Unspecified = 0,
    Long = 1,
    Short = 2,
}

impl PerpetualPosition {
    pub fn try_from_i32(value: i32) -> Result<Self, StdError> {
        match value {
            0 => Ok(Self::Unspecified),
            1 => Ok(Self::Long),
            2 => Ok(Self::Short),
            _ => Err(StdError::generic_err("PerpetualPosition out of range")),
        }
    }
}

impl ToString for PerpetualPosition {
    fn to_string(&self) -> String {
        match self {
            PerpetualPosition::Unspecified => "Unspecified".to_string(),
            PerpetualPosition::Long => "Long".to_string(),
            PerpetualPosition::Short => "Short".to_string(),
        }
    }
}

impl FromStr for PerpetualPosition {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Unspecified" => PerpetualPosition::Unspecified,
            "Long" => PerpetualPosition::Long,
            "Short" => PerpetualPosition::Short,
            _ => return Err(StdError::generic_err("unknow type")),
        })
    }
}

#[cw_serde]
pub struct Mtp {
    pub address: String,
    pub amm_pool_id: u64,
    pub borrow_interest_paid_collateral: Int128,
    pub borrow_interest_paid_custody: Int128,
    pub borrow_interest_unpaid_collateral: Int128,
    pub collateral_asset: String,
    pub collateral: Int128,
    pub consolidate_leverage: SignedDecimal,
    pub custody: Int128,
    pub custody_asset: String,
    pub funding_fee_paid_collateral: Int128,
    pub funding_fee_paid_custody: Int128,
    pub funding_fee_received_collateral: Int128,
    pub funding_fee_received_custody: Int128,
    pub id: u64,
    pub leverage: SignedDecimal,
    pub liabilities: Int128,
    pub liabilities_asset: String,
    pub mtp_health: SignedDecimal,
    pub open_price: SignedDecimal,
    pub position: i32,
    pub sum_collateral: Int128,
    pub take_profit_borrow_rate: SignedDecimal,
    pub take_profit_custody: Int128,
    pub take_profit_liabilities: Int128,
    pub take_profit_price: SignedDecimal256,
    pub trading_asset: String,
}

#[cw_serde]
pub enum EarnType {
    AllProgram = 0,
    UsdcProgram = 1,
    ElysProgram = 2,
    EdenProgram = 3,
    EdenBProgram = 4,
}

#[cw_serde]
pub struct BalanceAvailable {
    pub amount: Uint128,
    pub usd_amount: Decimal,
}

#[cw_serde]
pub struct VestingDetail {
    // The id of the vesting
    pub id: String,
    // The total vest for the current vest
    pub total_vest: BalanceAvailable,
    // The balance that's already vested
    pub balance_vested: BalanceAvailable,
    // The remaining amount to vest
    pub remaining_vest: BalanceAvailable,
    // Remaining time to vest. Javascript timestamp.
    pub remaining_time: u64,
}
#[cw_serde]
pub struct PoolParamsRaw {
    pub swap_fee: Decimal,
    pub exit_fee: Decimal,
    pub use_oracle: Option<bool>,
    pub weight_breaking_fee_multiplier: Decimal,
    pub weight_breaking_fee_exponent: Decimal,
    pub external_liquidity_ratio: Decimal,
    pub weight_recovery_fee_portion: Decimal,
    pub threshold_weight_difference: Decimal,
    pub fee_denom: Option<String>,
}

#[cw_serde]
pub struct PoolParams {
    pub swap_fee: Decimal,
    pub exit_fee: Decimal,
    pub use_oracle: bool,
    pub weight_breaking_fee_multiplier: Decimal,
    pub weight_breaking_fee_exponent: Decimal,
    pub external_liquidity_ratio: Decimal,
    pub weight_recovery_fee_portion: Decimal,
    pub threshold_weight_difference: Decimal,
    pub fee_denom: String,
}

impl From<PoolParamsRaw> for PoolParams {
    fn from(value: PoolParamsRaw) -> Self {
        Self {
            swap_fee: value.swap_fee,
            exit_fee: value.exit_fee,
            use_oracle: value.use_oracle.unwrap_or(false),
            weight_breaking_fee_multiplier: value.weight_breaking_fee_multiplier,
            weight_breaking_fee_exponent: value.weight_breaking_fee_exponent,
            external_liquidity_ratio: value.external_liquidity_ratio,
            weight_recovery_fee_portion: value.weight_recovery_fee_portion,
            threshold_weight_difference: value.threshold_weight_difference,
            fee_denom: value.fee_denom.unwrap_or("".to_string()),
        }
    }
}

#[cw_serde]
pub struct AmmPoolRaw {
    pub pool_id: Option<u64>,
    pub address: Option<String>,
    pub pool_params: PoolParamsRaw,
    pub total_shares: Coin,
    pub pool_assets: Vec<PoolAsset>,
    pub total_weight: Int128,
    pub rebalance_treasury: Option<String>,
}

impl From<AmmPoolRaw> for AmmPool {
    fn from(value: AmmPoolRaw) -> Self {
        Self {
            pool_id: value.pool_id.unwrap_or(0),
            address: value.address.unwrap_or("".to_string()),
            pool_params: value.pool_params,
            total_shares: value.total_shares,
            pool_assets: value.pool_assets,
            total_weight: value.total_weight,
            rebalance_treasury: value.rebalance_treasury.unwrap_or("".to_string()),
        }
    }
}

#[cw_serde]
pub struct AmmPool {
    pub pool_id: u64,
    pub address: String,
    pub pool_params: PoolParamsRaw,
    pub total_shares: Coin,
    pub pool_assets: Vec<PoolAsset>,
    pub total_weight: Int128,
    pub rebalance_treasury: String,
}

#[cw_serde]
pub struct PoolExtraInfo {
    tvl: Decimal,
    lp_token_price: Decimal,
}

#[cw_serde]
pub struct AmmJoinPool {
    pub sender: String,
    pub pool_id: u64,
    pub max_amounts_in: Vec<Coin>,
    pub share_amount_out: Uint128,
    pub no_remaining: bool,
}

#[cw_serde]
pub struct AmmExitPool {
    pub sender: String,
    pub pool_id: u64,
    pub min_amounts_out: Vec<Coin>,
    pub share_amount_in: Uint128,
    pub token_out_denom: String,
}

#[cw_serde]
pub struct StakingValidatorRaw {
    // The validator Identity.
    pub id: Option<String>,
    // The validator address.
    pub address: Option<String>,
    // The validator name.
    pub name: Option<String>,
    // Voting power percentage for this validator.
    pub voting_power: Option<Decimal>,
    // commission percentage for the validator.
    pub commission: Option<Decimal>,
}

#[cw_serde]
pub struct StakingValidator {
    // The validator Identity.
    pub id: String,
    // The validator address.
    pub address: String,
    // The validator name.
    pub name: String,
    // Voting power percentage for this validator.
    pub voting_power: Decimal,
    // commission percentage for the validator.
    pub commission: Decimal,
}

#[cw_serde]
pub struct StakedPositionRaw {
    // The position ID.
    pub id: String,
    // The validator that's being unstaked from.
    pub validator: StakingValidatorRaw,
    // The amount that's being staked.
    pub staked: BalanceAvailable,
}

#[cw_serde]
pub struct StakedPosition {
    // The position ID.
    pub id: String,
    // The validator that's being unstaked from.
    pub validator: StakingValidator,
    // The amount that's being staked.
    pub staked: BalanceAvailable,
}

#[cw_serde]
pub struct UnstakedPosition {
    // The position ID.
    pub id: String,
    // The validator that's being unstaked from.
    pub validator: StakingValidator,
    pub remaining_time: u64, // Remaining time to unstake in days.
    // The amount that's being staked.
    pub unstaked: BalanceAvailable,
}

#[cw_serde]
pub struct ValidatorDetail {
    // Validator Identity
    pub id: Option<String>,
    // The validator address.
    pub address: String,
    // The validator name.
    pub name: String,
    // Voting power percentage for this validator.
    pub voting_power: Decimal,
    // commission percentage for the validator.
    pub commission: Decimal,
    // The staked amount the user has w/ this validator
    // Only available if there's some and if address.
    // is sent in request object.
    pub staked: Option<BalanceAvailable>,
}

#[cw_serde]
pub struct PoolAsset {
    pub token: Coin,
    pub weight: Uint128,
    pub usd_value: Option<Decimal>,
}

pub fn default_take_profit_price() -> SignedDecimal256 {
    SignedDecimal256::new(
        Int256::from_str("10000000000000000000000000000000000000000000000000000000000").unwrap(),
    )
}
