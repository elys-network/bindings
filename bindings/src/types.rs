use cosmwasm_schema::cw_serde;
use cosmwasm_std::from_json;
use cosmwasm_std::to_json_binary;
use cosmwasm_std::Binary;
use cosmwasm_std::Coin;
use cosmwasm_std::Decimal;
use cosmwasm_std::Int128;
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
pub enum MarginPosition {
    Unspecified = 0,
    Long = 1,
    Short = 2,
}

impl MarginPosition {
    pub fn try_from_i32(value: i32) -> Result<Self, StdError> {
        match value {
            0 => Ok(Self::Unspecified),
            1 => Ok(Self::Long),
            2 => Ok(Self::Short),
            _ => Err(StdError::generic_err("MarginPosition out of range")),
        }
    }
}

#[cw_serde]
pub struct Mtp {
    pub address: String,
    pub collaterals: Vec<Coin>,
    pub liabilities: Int128,
    pub interest_paid_collaterals: Vec<Coin>,
    pub interest_paid_custodies: Vec<Coin>,
    pub interest_unpaid_collaterals: Vec<Coin>,
    pub custodies: Vec<Coin>,
    pub take_profit_liabilities: Int128,
    pub take_profit_custodies: Vec<Coin>,
    pub leverages: Vec<Decimal>,
    pub mtp_health: Decimal,
    pub position: i32,
    pub id: u64,
    pub amm_pool_id: u64,
    pub consolidate_leverage: Decimal,
    pub sum_collateral: Int128,
    pub take_profit_price: Decimal,
    pub funding_fee_paid_collaterals: Vec<Coin>,
    pub funding_fee_paid_custodies: Vec<Coin>,
    pub funding_fee_received_collaterals: Vec<Coin>,
    pub funding_fee_received_custodies: Vec<Coin>,
    pub collateral_asset: String,
    pub trading_asset: String,
    pub liabilities_asset: String,
    pub custody_asset: String,
    pub take_profit_borrow_rate: Decimal,
    pub open_price: Decimal,
}

#[cw_serde]
pub enum Sum {
    Ed25519(Binary),
    Secp256k1(Binary),
}

#[cw_serde]
pub struct PublicKey(Sum);

impl PublicKey {
    pub fn get_sum(&self) -> Sum {
        self.0.clone()
    }
}
#[cfg(feature = "testing")]
impl PublicKey {
    pub fn set(sum: Sum) -> Self {
        Self(sum)
    }
}

#[cw_serde]
pub struct BaseAccount {
    pub address: String,
    pub pub_key: PublicKey,
    pub account_number: u64,
    pub sequence: u64,
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
