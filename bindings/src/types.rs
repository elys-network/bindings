use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use cosmwasm_std::Coin;
use cosmwasm_std::Decimal;
use cosmwasm_std::StdError;

#[cw_serde]
pub struct AssetInfo {
    pub denom: String,
    pub display: String,
    pub band_ticker: String,
    pub elys_ticker: String,
    pub decimal: u64,
}

#[cfg(test)]
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
    // pub total: Option<u64>,
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

#[cw_serde]
pub struct SwapAmountInRoute {
    pub pool_id: u64,
    pub token_out_denom: String,
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
