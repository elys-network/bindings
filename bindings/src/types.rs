use cosmwasm_schema::cw_serde;
use cosmwasm_std::from_binary;
use cosmwasm_std::to_binary;
use cosmwasm_std::Binary;
use cosmwasm_std::Coin;
use cosmwasm_std::Decimal;
use cosmwasm_std::StdError;
use cosmwasm_std::StdResult;

#[cw_serde]
pub struct AssetInfo {
    pub denom: String,
    pub display: String,
    pub band_ticker: String,
    pub elys_ticker: String,
    pub decimal: u64,
}

#[cfg(feature = "testing")]
impl AssetInfo {
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
    key: Option<Binary>,
    offset: Option<u64>,
    limit: u64,
    count_total: bool,
    reverse: bool,
}

impl PageRequest {
    pub fn filter<T>(&self, static_vec: Vec<T>) -> StdResult<(Vec<T>, PageResponse)>
    where
        T: PartialEq,
        T: Clone,
    {
        let mut filter_vec = static_vec.clone();

        let key = if let Some(key) = &self.key {
            let key: u64 = from_binary(key)?;
            if key + 1 >= filter_vec.len() as u64 {
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

        let _ = filter_vec.split_off(self.limit as usize);

        if filter_vec.is_empty() {
            return Ok((vec![], PageResponse::empty(self.count_total)));
        };

        let next_key = if static_vec.last() == filter_vec.last() {
            None
        } else {
            Some(to_binary(&(key + self.limit + offset))?)
        };

        let total = if self.count_total {
            Some(filter_vec.len() as u64)
        } else {
            None
        };

        if self.reverse {
            filter_vec.reverse();
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
pub struct MarginOrder {
    pub order_id: u64,
    pub position: MarginPosition,
    pub collateral: Coin,
    pub borrow_token: Coin,
    pub creator: String,
    pub leverage: Decimal,
    pub take_profit_price: Decimal,
}

impl MarginOrder {
    pub fn new(
        position: MarginPosition,
        creator: impl Into<String>,
        collateral: Coin,
        leverage: Decimal,
        borrow_token: Coin,
        take_profit_price: Decimal,
    ) -> Self {
        let order_id: u64 = 0;

        Self {
            order_id,
            position,
            collateral,
            borrow_token,
            creator: creator.into(),
            leverage,
            take_profit_price,
        }
    }
}
